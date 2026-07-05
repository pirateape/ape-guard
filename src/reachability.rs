// ApeGuard Reachability Analysis
// Determines which source files are transitively reachable from known entry points,
// enabling detection of dead-code findings (unreachable = lower risk).
//
// Architecture:
//   1. discover_entry_points() → collect known and user-specified entry point files
//   2. collect_source_files() → gather all source files by language extension
//   3. build_import_graph() → regex-based import extraction → adjacency list
//   4. bfs_reachable_files() → transitive closure from entry points
//   5. apply_reachability() → set reachable field on each finding
//
// Pipeline insertion point: after dedup, before FP filter (in main.rs)
use crate::find::CanonicalFinding;
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// Results of a reachability analysis pass
#[derive(Debug, Clone)]
#[expect(dead_code)] // P3/P4: ReachabilityResult fields populated by reachability analysis; not all fields read yet
pub struct ReachabilityResult {
    /// Whether analysis was actually performed
    pub enabled: bool,
    /// Set of file paths found to be transitively reachable from entry points
    pub reachable_files: HashSet<PathBuf>,
    /// All entry points discovered and used for the analysis
    pub entry_points: Vec<PathBuf>,
    /// Number of files in the import graph
    pub total_files_analyzed: usize,
    /// Number of edges in the import graph
    pub total_imports: usize,
    /// Number of findings marked as reachable
    pub findings_reachable: usize,
    /// Number of findings marked as unreachable
    pub findings_unreachable: usize,
    /// Number of findings with unknown reachability (file not in graph or no location)
    pub findings_unknown: usize,
}

/// Configuration for reachability analysis
#[derive(Debug, Clone)]
pub struct ReachabilityConfig {
    /// Master switch — opt-in (default: false)
    pub enabled: bool,
    /// User-specified entry point globs (e.g., "src/main.rs", "bin/*.rs")
    pub entry_points: Vec<String>,
    /// File extensions to include in analysis
    pub include_extensions: Vec<String>,
    /// Directories to exclude from analysis
    pub exclude_dirs: Vec<String>,
}

impl Default for ReachabilityConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            entry_points: vec![],
            include_extensions: vec![
                "rs".to_string(),
                "py".to_string(),
                "js".to_string(),
                "ts".to_string(),
                "tsx".to_string(),
                "jsx".to_string(),
                "go".to_string(),
                "c".to_string(),
                "cpp".to_string(),
                "h".to_string(),
                "hpp".to_string(),
            ],
            exclude_dirs: vec![
                ".git".to_string(),
                "node_modules".to_string(),
                "target".to_string(),
                ".apeguard".to_string(),
                "vendor".to_string(),
                "__pycache__".to_string(),
                ".venv".to_string(),
                "venv".to_string(),
                "dist".to_string(),
                "build".to_string(),
            ],
        }
    }
}

// ─── Entry Point Discovery ───────────────────────────────────────────────────

/// Automatically discover entry point files in the target directory.
/// Checks for well-known entry points by language convention.
fn discover_entry_points(target: &Path, user_entries: &[String]) -> Vec<PathBuf> {
    let mut entries: Vec<PathBuf> = Vec::new();

    // User-specified entry points (globs are expanded elsewhere, these are direct paths)
    for path_str in user_entries {
        let p = target.join(path_str);
        if p.exists() && p.is_file() {
            entries.push(canonicalize_path(&p));
        }
    }

    // Language-specific auto-discovery
    let known_entries = [
        // Rust
        "src/main.rs",
        "src/lib.rs",
        "main.rs",
        "lib.rs",
        // Python
        "main.py",
        "app.py",
        "cli.py",
        "__main__.py",
        // JavaScript / TypeScript
        "index.js",
        "index.ts",
        "main.js",
        "main.ts",
        "src/index.js",
        "src/index.ts",
        "src/main.js",
        "src/main.ts",
        "src/app.js",
        "src/app.ts",
        "src/index.tsx",
        "src/main.tsx",
        "src/app.tsx",
        // Go
        "main.go",
        "cmd/main.go",
        // Deno
        "mod.ts",
        "mod.js",
        "src/mod.ts",
        // C/C++
        "main.c",
        "main.cpp",
    ];

    for entry in &known_entries {
        let p = target.join(entry);
        if p.exists() && p.is_file() {
            entries.push(canonicalize_path(&p));
        }
    }

    // Deduplicate while preserving order
    let mut seen = HashSet::new();
    entries.retain(|e| seen.insert(e.clone()));

    entries
}

// ─── Source File Collection ──────────────────────────────────────────────────

/// Collect all source files in the target directory matching the given extensions,
/// excluding common vendor/build directories.
fn collect_source_files(
    target: &Path,
    extensions: &[String],
    exclude_dirs: &[String],
) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let exclude_set: HashSet<&str> = exclude_dirs.iter().map(|s| s.as_str()).collect();

    collect_files_recursive(target, target, &mut files, extensions, &exclude_set);

    files
}

#[expect(clippy::only_used_in_recursion)]
fn collect_files_recursive(
    root: &Path,
    dir: &Path,
    files: &mut Vec<PathBuf>,
    extensions: &[String],
    exclude_dirs: &HashSet<&str>,
) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries {
        let Ok(entry) = entry else {
            continue;
        };
        let path = entry.path();

        if path.is_dir() {
            // Skip excluded directories
            if let Some(dirname) = path.file_name().and_then(|n| n.to_str()) {
                if exclude_dirs.contains(dirname) {
                    continue;
                }
            }
            collect_files_recursive(root, &path, files, extensions, exclude_dirs);
        } else if path.is_file() {
            // Check extension
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if extensions.iter().any(|e| e == ext) {
                    files.push(canonicalize_path(&path));
                }
            }
        }
    }
}

// ─── Import Graph Construction ───────────────────────────────────────────────

/// Extract import statements from a source file and resolve them to local file paths.
/// Returns a list of file paths that this file depends on (within the target directory).
fn extract_imports(file_path: &Path, target_root: &Path) -> Vec<PathBuf> {
    let content = match std::fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(_) => return vec![],
    };

    let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let parent = file_path.parent().unwrap_or(target_root);
    let mut imports: Vec<PathBuf> = Vec::new();

    match ext {
        "rs" => extract_rust_imports(&content, parent, target_root, &mut imports),
        "py" => extract_python_imports(&content, parent, target_root, &mut imports),
        "js" | "ts" | "tsx" | "jsx" => {
            extract_js_imports(&content, parent, target_root, &mut imports)
        }
        "go" => extract_go_imports(&content, parent, target_root, &mut imports),
        "c" | "cpp" | "h" | "hpp" => extract_c_imports(&content, parent, target_root, &mut imports),
        _ => {}
    }

    imports
}

/// Rust imports: `use crate::foo::bar`, `mod foo`, `use module::path`
fn extract_rust_imports(
    content: &str,
    parent: &Path,
    target_root: &Path,
    imports: &mut Vec<PathBuf>,
) {
    for line in content.lines() {
        let trimmed = line.trim();

        // `mod module_name;` — inline module declaration
        if let Some(name) = trimmed
            .strip_prefix("mod ")
            .and_then(|s| s.strip_suffix(';'))
        {
            let name = name.trim();
            if !name.contains("::") && !name.contains('"') && !name.contains(char::is_whitespace) {
                // Try as file: module_name.rs or module_name/mod.rs
                for candidate in [
                    parent.join(format!("{name}.rs")),
                    parent.join(format!("{name}/mod.rs")),
                ] {
                    if candidate.exists() {
                        imports.push(canonicalize_path(&candidate));
                        break;
                    }
                }
            }
        }

        // `use crate::...` — crate-internal absolute path
        if let Some(rest) = trimmed.strip_prefix("use crate::") {
            let path = rest.trim_end_matches(';');
            // Convert "use crate::foo::bar" → "src/foo/bar.rs" or "src/foo.rs"
            let parts: Vec<&str> = path.split("::").collect();
            if let Some(relative) = resolve_rust_crate_path(&parts, target_root) {
                if relative.exists() {
                    imports.push(canonicalize_path(&relative));
                }
            }
        }

        // `use super::...` — parent module path
        if trimmed.starts_with("use super::") || trimmed.starts_with("use self::") {
            // These refer to relative paths within the module tree
            // Resolve relative to the file's position in the source tree
            resolve_relative_use(trimmed, parent, target_root, imports);
        }
    }
}

/// Resolve a `use crate::foo::bar` path to a file path relative to target_root
fn resolve_rust_crate_path(parts: &[&str], target_root: &Path) -> Option<PathBuf> {
    if parts.is_empty() {
        return None;
    }

    // Try as directory/mod.rs pattern first: src/foo/bar/mod.rs
    let mut dir_path = target_root.join("src");
    for part in parts {
        dir_path = dir_path.join(part);
    }
    let mod_rs = dir_path.join("mod.rs");
    if mod_rs.exists() {
        return Some(mod_rs);
    }

    // Try as file: src/foo.rs (last part is the module name)
    let mut file_path = target_root.join("src");
    for (i, part) in parts.iter().enumerate() {
        if i < parts.len() - 1 {
            file_path = file_path.join(part);
        } else {
            // Last part is the actual module
            if file_path.join(format!("{part}.rs")).exists() {
                return Some(file_path.join(format!("{part}.rs")));
            }
            // Also try as directory/mod.rs
            if file_path.join(part).join("mod.rs").exists() {
                return Some(file_path.join(part).join("mod.rs"));
            }
        }
    }

    None
}

/// Resolve `use super::` and `use self::` relative paths
fn resolve_relative_use(
    trimmed: &str,
    parent: &Path,
    target_root: &Path,
    imports: &mut Vec<PathBuf>,
) {
    // Strip "use " prefix
    let path = trimmed.strip_prefix("use ").unwrap_or(trimmed);
    let path = path.trim_end_matches(';');

    // Count `super::` segments to go up
    let mut depth = 0;
    let mut rest = path;
    while let Some(suffix) = rest.strip_prefix("super::") {
        depth += 1;
        rest = suffix;
    }
    rest = rest.strip_prefix("self::").unwrap_or(rest);

    // Convert path segments to file path
    let segments: Vec<&str> = rest.split("::").collect();
    if segments.is_empty() || segments[0].is_empty() {
        return;
    }

    // Start from the appropriate parent directory
    let mut base = parent.to_path_buf();
    for _ in 0..depth {
        base = base
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| target_root.to_path_buf());
    }

    // Try as file: base/{last}.rs
    if let Some(last) = segments.last() {
        let mut dir = base.clone();
        for seg in segments.iter().take(segments.len() - 1) {
            dir = dir.join(seg);
        }
        let as_file = dir.join(format!("{last}.rs"));
        if as_file.exists() {
            imports.push(canonicalize_path(&as_file));
            return;
        }
        // Try as directory/mod.rs
        let as_dir_module = dir.join(last).join("mod.rs");
        if as_dir_module.exists() {
            imports.push(canonicalize_path(&as_dir_module));
            return;
        }
        // Try as directory/{last}/mod.rs where last is the last segment
        let full_dir = dir.join(last);
        let as_dir_mod = full_dir.join("mod.rs");
        if as_dir_mod.exists() {
            imports.push(canonicalize_path(&as_dir_mod));
        }
    }
}

/// Python imports: `import module`, `from module import name`
fn extract_python_imports(
    content: &str,
    parent: &Path,
    target_root: &Path,
    imports: &mut Vec<PathBuf>,
) {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // `from .module import name` — relative import
        if let Some(rest) = trimmed.strip_prefix("from ") {
            if let Some(module_path) = rest.split_whitespace().next() {
                if module_path.starts_with('.') {
                    // Relative import
                    let relative = resolve_python_relative(module_path, parent, target_root);
                    imports.extend(relative);
                } else {
                    // Absolute import — try to resolve within the project
                    let resolved = resolve_python_absolute(module_path, target_root);
                    imports.extend(resolved);
                }
            }
        }

        // `import module` — absolute import
        if let Some(rest) = trimmed.strip_prefix("import ") {
            let module_name = rest.split_whitespace().next().unwrap_or("");
            let resolved = resolve_python_absolute(module_name, target_root);
            imports.extend(resolved);
        }
    }
}

fn resolve_python_relative(module_path: &str, parent: &Path, target_root: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    // Count leading dots to determine parent depth
    let dot_count = module_path.chars().take_while(|c| *c == '.').count();
    let module_name = &module_path[dot_count..];

    let mut base = parent.to_path_buf();
    for _ in 1..dot_count {
        base = base
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| target_root.to_path_buf());
    }

    // Try module.py and module/__init__.py
    let file = base.join(format!("{module_name}.py"));
    if file.exists() {
        result.push(canonicalize_path(&file));
    }
    let init = base.join(module_name).join("__init__.py");
    if init.exists() {
        result.push(canonicalize_path(&init));
    }

    result
}

fn resolve_python_absolute(module_name: &str, target_root: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    let parts: Vec<&str> = module_name.split('.').collect();

    // Try target_root/{parts_as_path}.py
    let mut path = target_root.to_path_buf();
    for (i, part) in parts.iter().enumerate() {
        if i < parts.len() - 1 {
            path = path.join(part);
        } else {
            // Try as file
            let file = path.join(format!("{part}.py"));
            if file.exists() {
                result.push(canonicalize_path(&file));
            }
            // Try as package
            let init = path.join(part).join("__init__.py");
            if init.exists() {
                result.push(canonicalize_path(&init));
            }
        }
    }

    // Also try from src/ subdirectory
    let src_path = target_root.join("src");
    if src_path.exists() {
        let mut path = src_path;
        for (i, part) in parts.iter().enumerate() {
            if i < parts.len() - 1 {
                path = path.join(part);
            } else {
                let file = path.join(format!("{part}.py"));
                if file.exists() {
                    result.push(canonicalize_path(&file));
                }
                let init = path.join(part).join("__init__.py");
                if init.exists() {
                    result.push(canonicalize_path(&init));
                }
            }
        }
    }

    result
}

/// JavaScript/TypeScript imports: `import ... from './path'`, `require('./path')`
fn extract_js_imports(
    content: &str,
    parent: &Path,
    target_root: &Path,
    imports: &mut Vec<PathBuf>,
) {
    // Lines containing import/require with relative paths
    for line in content.lines() {
        let trimmed = line.trim();

        // `import ... from './path'` or `import ... from "../path"`
        // Also `import './path'`
        for cap in trimmed
            .match_indices("from ")
            .chain(trimmed.match_indices("import "))
        {
            // Find the string literal after from/import
            let after = &trimmed[cap.0 + cap.1.len()..];
            if let Some(path_str) = extract_quoted_string(after) {
                if path_str.starts_with("./") || path_str.starts_with("../") {
                    resolve_js_path(&path_str, parent, target_root, imports);
                }
            }
        }

        // `require('./path')`
        if let Some(rest) = trimmed.find("require(") {
            let after = &trimmed[rest + 8..];
            if let Some(path_str) = extract_quoted_string(after) {
                if path_str.starts_with("./") || path_str.starts_with("../") {
                    resolve_js_path(&path_str, parent, target_root, imports);
                }
            }
        }
    }
}

fn extract_quoted_string(s: &str) -> Option<String> {
    let s = s.trim();
    // Handle both single and double quotes
    let quote = s.chars().next()?;
    if quote != '\'' && quote != '"' {
        return None;
    }
    let end = s[1..].find(quote)?;
    Some(s[1..=end].to_string())
}

fn resolve_js_path(path_str: &str, parent: &Path, _target_root: &Path, imports: &mut Vec<PathBuf>) {
    let resolved = parent.join(path_str);

    // Try exact path — only if it's a file, not a directory
    if resolved.is_file() {
        imports.push(canonicalize_path(&resolved));
        return;
    }

    // Try with extensions
    for ext in &[
        ".js", ".ts", ".tsx", ".jsx", ".mjs", ".cjs", ".mts", ".cts", ".json",
    ] {
        let with_ext = resolved.with_extension(ext.strip_prefix('.').unwrap_or(ext));
        if with_ext.is_file() {
            imports.push(canonicalize_path(&with_ext));
            return;
        }
    }

    // Try as directory with index file
    for index_file in &[
        "index.js",
        "index.ts",
        "index.tsx",
        "index.jsx",
        "index.mjs",
    ] {
        let index = resolved.join(index_file);
        if index.is_file() {
            imports.push(canonicalize_path(&index));
            return;
        }
    }
}

/// Go imports: `import "path"`, `import ("path1"\n"path2")`
fn extract_go_imports(
    _content: &str,
    _parent: &Path,
    _target_root: &Path,
    _imports: &mut Vec<PathBuf>,
) {
    // Go uses full module paths (e.g., "github.com/user/repo/pkg"),
    // not relative file paths. Local imports use the module name.
    // For V1, we skip Go import resolution (too complex without AST).
    // Entry points are still discovered via main.go — just no import following.
}

/// C/C++ includes: `#include "header.h"` (relative includes only)
fn extract_c_imports(content: &str, parent: &Path, target_root: &Path, imports: &mut Vec<PathBuf>) {
    for line in content.lines() {
        let trimmed = line.trim();

        // `#include "header.h"` — quoted includes are relative
        if let Some(rest) = trimmed.strip_prefix("#include ") {
            let rest = rest.trim();
            if rest.starts_with('"') {
                let header = rest.trim_matches('"');
                let path = parent.join(header);
                if path.exists() {
                    imports.push(canonicalize_path(&path));
                } else {
                    // Also try from target root
                    let root_path = target_root.join(header);
                    if root_path.exists() {
                        imports.push(canonicalize_path(&root_path));
                    }
                }
            }
        }
    }
}

// ─── Import Graph and BFS ────────────────────────────────────────────────────

/// Adjacency list: file → list of files it imports
/// Uses `Rc<PathBuf>` so graph construction and BFS share path data without cloning.
type ImportGraph = HashMap<Rc<PathBuf>, Vec<Rc<PathBuf>>>;

/// Build a dependency graph as an adjacency list from source files to their imports.
/// Returns (graph, reverse_graph, total_imports_count).
fn build_import_graph(
    all_files: &[PathBuf],
    target_root: &Path,
) -> (ImportGraph, ImportGraph, usize) {
    let mut graph: ImportGraph = HashMap::new();
    let mut reverse: ImportGraph = HashMap::new();
    let mut total_imports = 0;

    // Wrap each PathBuf in Rc once — cheap clones in hot paths
    let rc_files: Vec<Rc<PathBuf>> = all_files.iter().map(|f| Rc::new(f.clone())).collect();
    let path_to_rc: HashMap<&Path, &Rc<PathBuf>> =
        rc_files.iter().map(|rc| (rc.as_path(), rc)).collect();

    // Pre-populate with all files
    for file in &rc_files {
        graph.entry(Rc::clone(file)).or_default();
        reverse.entry(Rc::clone(file)).or_default();
    }

    // Extract imports for each file
    for file in &rc_files {
        let imports = extract_imports(file, target_root);
        for import in &imports {
            if let Some(rc_import) = path_to_rc.get(import.as_path()) {
                // rc_import is &&Rc<PathBuf>; *rc_import gives &Rc<PathBuf>
                graph
                    .get_mut(file)
                    .expect("file must be pre-populated in graph")
                    .push(Rc::clone(*rc_import));
                reverse
                    .get_mut(*rc_import)
                    .expect("import must be pre-populated in reverse graph")
                    .push(Rc::clone(file));
                total_imports += 1;
            }
        }
    }

    (graph, reverse, total_imports)
}

/// BFS from all entry points through the import graph.
/// Returns the set of transitively reachable files.
fn bfs_reachable_files(graph: &ImportGraph, entry_points: &[PathBuf]) -> HashSet<PathBuf> {
    let mut reachable: HashSet<&Rc<PathBuf>> = HashSet::new();
    let mut queue: VecDeque<&Rc<PathBuf>> = VecDeque::new();

    // Seed with entry points — find matching Rc in the graph by path content
    for entry in entry_points {
        if let Some(rc_entry) = graph.keys().find(|k| k.as_path() == entry.as_path()) {
            if !reachable.contains(rc_entry) {
                reachable.insert(rc_entry);
                queue.push_back(rc_entry);
            }
        }
    }

    // BFS
    while let Some(current) = queue.pop_front() {
        if let Some(deps) = graph.get(current) {
            for dep in deps {
                if !reachable.contains(dep) {
                    reachable.insert(dep);
                    queue.push_back(dep);
                }
            }
        }
    }

    // Convert back — only clone PathBuf data at the return boundary
    reachable.into_iter().map(|rc| (**rc).clone()).collect()
}

// ─── Core Public API ─────────────────────────────────────────────────────────

/// Run the full reachability analysis pipeline.
///
/// 1. Discover entry points (auto + user-specified)
/// 2. Collect source files
/// 3. Build import graph
/// 4. BFS to find reachable files
/// 5. Mark findings
///
/// Returns a `ReachabilityResult` with full statistics.
pub fn analyze_reachability(
    findings: &[CanonicalFinding],
    target: &Path,
    config: &ReachabilityConfig,
) -> ReachabilityResult {
    if !config.enabled {
        return ReachabilityResult {
            enabled: false,
            reachable_files: HashSet::new(),
            entry_points: vec![],
            total_files_analyzed: 0,
            total_imports: 0,
            findings_reachable: 0,
            findings_unreachable: 0,
            findings_unknown: findings.len(),
        };
    }

    // Step 1: Discover entry points
    let entry_points = discover_entry_points(target, &config.entry_points);
    tracing::info!(
        "Reachability: found {} entry point(s) in {}",
        entry_points.len(),
        target.display()
    );
    for ep in &entry_points {
        tracing::debug!("  Entry point: {}", ep.display());
    }

    // Step 2: Collect source files
    let all_files = collect_source_files(target, &config.include_extensions, &config.exclude_dirs);
    tracing::info!(
        "Reachability: collected {} source files for analysis",
        all_files.len()
    );

    if all_files.is_empty() || entry_points.is_empty() {
        return ReachabilityResult {
            enabled: true,
            reachable_files: HashSet::new(),
            entry_points,
            total_files_analyzed: all_files.len(),
            total_imports: 0,
            findings_reachable: 0,
            findings_unreachable: 0,
            findings_unknown: findings.len(),
        };
    }

    // Step 3: Build import graph
    let (graph, _, total_imports) = build_import_graph(&all_files, target);

    // Step 4: BFS reachability
    let reachable_files = bfs_reachable_files(&graph, &entry_points);

    tracing::info!(
        "Reachability: {} of {} files are reachable from entry points ({} imports resolved)",
        reachable_files.len(),
        all_files.len(),
        total_imports
    );

    #[allow(clippy::mutable_key_type)]
    let reachable_canon: HashSet<PathBuf> = reachable_files
        .iter()
        .map(|p| canonicalize_path(p))
        .collect();

    ReachabilityResult {
        enabled: true,
        reachable_files: reachable_canon,
        entry_points,
        total_files_analyzed: all_files.len(),
        total_imports,
        findings_reachable: 0,
        findings_unreachable: 0,
        findings_unknown: 0,
    }
}

/// Apply reachability results to a mutable findings list.
/// Sets `finding.reachable = Some(true/false)` based on file path membership.
pub fn apply_reachability(findings: &mut [CanonicalFinding], result: &ReachabilityResult) {
    let mut reachable_count = 0usize;
    let mut unreachable_count = 0usize;
    let mut unknown_count = 0usize;

    for finding in findings.iter_mut() {
        let file_path = &finding.location.file;

        // Skip findings without a meaningful file path
        let file_str = file_path.to_string_lossy();
        if file_str.is_empty() || file_str == "." || file_str == "-" {
            finding.reachable = None;
            unknown_count += 1;
            continue;
        }

        let canonical = canonicalize_path(file_path);

        if result.reachable_files.contains(&canonical) {
            finding.reachable = Some(true);
            reachable_count += 1;
        } else if result.reachable_files.is_empty() && result.total_files_analyzed == 0 {
            // Analysis was not run — leave as None
            finding.reachable = None;
            unknown_count += 1;
        } else {
            // File exists in the graph but is not reachable
            finding.reachable = Some(false);
            unreachable_count += 1;
        }
    }

    tracing::info!(
        "Reachability applied: {} reachable, {} unreachable, {} unknown",
        reachable_count,
        unreachable_count,
        unknown_count,
    );
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Canonicalize a path: resolve to absolute, handle symlinks, normalize.
fn canonicalize_path(path: &Path) -> PathBuf {
    if path.is_absolute() {
        std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
    } else {
        // For relative paths, try to resolve relative to current dir
        if let Ok(cwd) = std::env::current_dir() {
            let absolute = cwd.join(path);
            std::fs::canonicalize(&absolute).unwrap_or(absolute)
        } else {
            path.to_path_buf()
        }
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn create_temp_dir() -> tempfile::TempDir {
        tempfile::tempdir().expect("failed to create temp dir")
    }

    fn write_file(dir: &Path, rel: &str, content: &str) -> PathBuf {
        let path = dir.join(rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn test_discover_entry_points_rust() {
        let tmp = create_temp_dir();
        write_file(tmp.path(), "src/main.rs", "fn main() {}");
        write_file(tmp.path(), "src/lib.rs", "pub fn helper() {}");

        let entries = discover_entry_points(tmp.path(), &[]);
        assert!(entries
            .iter()
            .any(|p| p.to_string_lossy().ends_with("main.rs")));
        assert!(entries
            .iter()
            .any(|p| p.to_string_lossy().ends_with("lib.rs")));
    }

    #[test]
    fn test_discover_entry_points_python() {
        let tmp = create_temp_dir();
        write_file(tmp.path(), "app.py", "def main(): pass");
        write_file(tmp.path(), "cli.py", "if __name__ == '__main__': pass");

        let entries = discover_entry_points(tmp.path(), &[]);
        assert!(!entries.is_empty());
    }

    #[test]
    fn test_discover_entry_points_none() {
        let tmp = create_temp_dir();
        // No entry points exist
        let entries = discover_entry_points(tmp.path(), &[]);
        assert!(entries.is_empty());
    }

    #[test]
    fn test_collect_source_files() {
        let tmp = create_temp_dir();
        write_file(tmp.path(), "src/main.rs", "fn main() {}");
        write_file(tmp.path(), "src/helper.rs", "pub fn helper() {}");
        write_file(tmp.path(), "src/lib.py", "def foo(): pass");
        write_file(tmp.path(), "README.md", "# Project");

        let files = collect_source_files(
            tmp.path(),
            &["rs".to_string(), "py".to_string()],
            &[".git".to_string(), "node_modules".to_string()],
        );
        assert_eq!(files.len(), 3); // 2 .rs + 1 .py
    }

    #[test]
    fn test_collect_source_files_excludes_common_dirs() {
        let tmp = create_temp_dir();
        write_file(
            tmp.path(),
            "node_modules/pkg/index.js",
            "module.exports = {}",
        );
        write_file(tmp.path(), "src/app.js", "console.log('hello')");

        let files = collect_source_files(
            tmp.path(),
            &["js".to_string()],
            &["node_modules".to_string(), ".git".to_string()],
        );
        // Should only find src/app.js, not node_modules
        assert_eq!(files.len(), 1);
        assert!(files[0].to_string_lossy().ends_with("app.js"));
    }

    #[test]
    fn test_extract_rust_mod_imports() {
        let tmp = create_temp_dir();
        write_file(tmp.path(), "src/module_a.rs", "pub fn a() {}");
        let main_rs = write_file(
            tmp.path(),
            "src/main.rs",
            "mod module_a;\nfn main() { module_a::a(); }",
        );

        let imports = extract_imports(&main_rs, tmp.path());
        assert!(
            imports
                .iter()
                .any(|p| p.to_string_lossy().ends_with("module_a.rs")),
            "Should find mod module_a import, got: {:?}",
            imports
        );
    }

    #[test]
    fn test_extract_rust_crate_imports() {
        let tmp = create_temp_dir();
        write_file(tmp.path(), "src/module_a.rs", "pub fn a() {}");
        let main_rs = write_file(
            tmp.path(),
            "src/main.rs",
            "use crate::module_a;\nfn main() { module_a::a(); }",
        );

        let imports = extract_imports(&main_rs, tmp.path());
        assert!(
            imports
                .iter()
                .any(|p| p.to_string_lossy().ends_with("module_a.rs")),
            "Should resolve use crate::module_a, got: {:?}",
            imports
        );
    }

    #[test]
    fn test_extract_rust_submodule_imports() {
        let tmp = create_temp_dir();
        write_file(tmp.path(), "src/utils/helpers.rs", "pub fn helper() {}");
        let main_rs = write_file(
            tmp.path(),
            "src/main.rs",
            "use crate::utils::helpers;\nfn main() { helpers::helper(); }",
        );

        let imports = extract_imports(&main_rs, tmp.path());
        assert!(
            imports
                .iter()
                .any(|p| p.to_string_lossy().ends_with("helpers.rs")),
            "Should resolve use crate::utils::helpers"
        );
    }

    #[test]
    fn test_extract_python_imports() {
        let tmp = create_temp_dir();
        write_file(tmp.path(), "utils.py", "def helper(): pass");
        let main_py = write_file(tmp.path(), "main.py", "import utils\nutils.helper()");

        let imports = extract_imports(&main_py, tmp.path());
        assert!(imports
            .iter()
            .any(|p| p.to_string_lossy().ends_with("utils.py")));
    }

    #[test]
    fn test_extract_python_from_import() {
        let tmp = create_temp_dir();
        write_file(tmp.path(), "utils/__init__.py", "");
        write_file(tmp.path(), "utils/helpers.py", "def helper(): pass");
        let main_py = write_file(tmp.path(), "main.py", "from utils.helpers import helper");

        let imports = extract_imports(&main_py, tmp.path());
        assert!(imports
            .iter()
            .any(|p| p.to_string_lossy().ends_with("helpers.py")
                || p.to_string_lossy().ends_with("__init__.py")));
    }

    #[test]
    fn test_extract_python_relative_import() {
        let tmp = create_temp_dir();
        write_file(tmp.path(), "package/__init__.py", "");
        write_file(tmp.path(), "package/module.py", "def foo(): pass");
        write_file(
            tmp.path(),
            "package/sub/__init__.py",
            "from ..module import foo",
        );

        let sub_init = tmp.path().join("package/sub/__init__.py");
        let imports = extract_imports(&sub_init, tmp.path());
        assert!(
            imports
                .iter()
                .any(|p| p.to_string_lossy().ends_with("module.py")),
            "Should resolve relative import ..module"
        );
    }

    #[test]
    fn test_extract_js_imports() {
        let tmp = create_temp_dir();
        write_file(tmp.path(), "lib/helper.js", "module.exports = {};");
        let main_js = write_file(
            tmp.path(),
            "src/index.js",
            "import { helper } from '../lib/helper';\nhelper();",
        );

        let imports = extract_imports(&main_js, tmp.path());
        assert!(imports
            .iter()
            .any(|p| p.to_string_lossy().ends_with("helper.js")));
    }

    #[test]
    fn test_extract_js_require() {
        let tmp = create_temp_dir();
        write_file(tmp.path(), "config.js", "module.exports = {};");
        let main_js = write_file(
            tmp.path(),
            "index.js",
            "const config = require('./config');",
        );

        let imports = extract_imports(&main_js, tmp.path());
        assert!(imports
            .iter()
            .any(|p| p.to_string_lossy().ends_with("config.js")));
    }

    #[test]
    fn test_extract_js_index_resolution() {
        let tmp = create_temp_dir();
        write_file(tmp.path(), "utils/index.js", "module.exports = {};");
        let main_js = write_file(tmp.path(), "index.js", "const utils = require('./utils');");

        let imports = extract_imports(&main_js, tmp.path());
        assert!(imports
            .iter()
            .any(|p| p.to_string_lossy().ends_with("index.js")));
    }

    #[test]
    fn test_extract_c_include() {
        let tmp = create_temp_dir();
        write_file(tmp.path(), "include/helpers.h", "#ifndef HELPERS_H");
        let main_c = write_file(
            tmp.path(),
            "src/main.c",
            "#include \"../include/helpers.h\"\nint main() { return 0; }",
        );

        let imports = extract_imports(&main_c, tmp.path());
        assert!(imports
            .iter()
            .any(|p| p.to_string_lossy().ends_with("helpers.h")));
    }

    #[test]
    fn test_bfs_simple() {
        let mut graph: ImportGraph = HashMap::new();
        let a = Rc::new(PathBuf::from("/a.rs"));
        let b = Rc::new(PathBuf::from("/b.rs"));
        let c = Rc::new(PathBuf::from("/c.rs"));

        graph.insert(Rc::clone(&a), vec![Rc::clone(&b)]);
        graph.insert(Rc::clone(&b), vec![Rc::clone(&c)]);
        graph.insert(Rc::clone(&c), vec![]);

        let reachable = bfs_reachable_files(&graph, &[(*a).clone()]);
        assert!(reachable.contains::<PathBuf>(&a));
        assert!(reachable.contains::<PathBuf>(&b));
        assert!(reachable.contains::<PathBuf>(&c));
        assert_eq!(reachable.len(), 3);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph: ImportGraph = HashMap::new();
        let a = Rc::new(PathBuf::from("/a.rs"));
        let b = Rc::new(PathBuf::from("/b.rs"));

        graph.insert(Rc::clone(&a), vec![Rc::clone(&b)]);
        graph.insert(Rc::clone(&b), vec![Rc::clone(&a)]); // Cycle back

        let reachable = bfs_reachable_files(&graph, &[(*a).clone()]);
        assert!(reachable.contains::<PathBuf>(&a));
        assert!(reachable.contains::<PathBuf>(&b));
        assert_eq!(reachable.len(), 2);
    }

    #[test]
    fn test_bfs_unreachable() {
        let mut graph: ImportGraph = HashMap::new();
        let a = Rc::new(PathBuf::from("/a.rs"));
        let b = Rc::new(PathBuf::from("/b.rs"));
        let c = Rc::new(PathBuf::from("/c.rs"));

        graph.insert(Rc::clone(&a), vec![Rc::clone(&b)]);
        graph.insert(Rc::clone(&b), vec![]);
        graph.insert(Rc::clone(&c), vec![]);

        let reachable = bfs_reachable_files(&graph, &[(*a).clone()]);
        assert_eq!(reachable.len(), 2); // a and b reachable, c not
        assert!(!reachable.contains::<PathBuf>(&c));
    }

    #[test]
    fn test_bfs_multiple_entry_points() {
        let mut graph: ImportGraph = HashMap::new();
        let main = Rc::new(PathBuf::from("/main.rs"));
        let lib = Rc::new(PathBuf::from("/lib.rs"));
        let shared = Rc::new(PathBuf::from("/shared.rs"));
        let unreachable = Rc::new(PathBuf::from("/dead.rs"));

        graph.insert(Rc::clone(&main), vec![Rc::clone(&shared)]);
        graph.insert(Rc::clone(&lib), vec![Rc::clone(&shared)]);
        graph.insert(Rc::clone(&shared), vec![]);
        graph.insert(Rc::clone(&unreachable), vec![]);

        let reachable = bfs_reachable_files(&graph, &[(*main).clone(), (*lib).clone()]);
        assert_eq!(reachable.len(), 3);
        assert!(!reachable.contains::<PathBuf>(&unreachable));
    }

    #[test]
    fn test_analyze_reachability_disabled() {
        let findings = vec![];
        let tmp = create_temp_dir();
        let config = ReachabilityConfig::default(); // enabled = false

        let result = analyze_reachability(&findings, tmp.path(), &config);
        assert!(!result.enabled);
        assert!(result.entry_points.is_empty());
        assert_eq!(result.total_files_analyzed, 0);
        assert_eq!(result.reachable_files.len(), 0);
    }

    #[test]
    fn test_apply_reachability_markings() {
        let tmp = create_temp_dir();
        let reachable_file = write_file(tmp.path(), "src/main.rs", "fn main() {}");
        let unreachable_file = write_file(tmp.path(), "src/dead.rs", "fn dead() {}");
        let reachable_canon = canonicalize_path(&reachable_file);

        let mut findings = vec![
            CanonicalFinding {
                id: "F-001".into(),
                location: crate::find::FindingLocation {
                    file: reachable_file.clone(),
                    line: None,
                    column: None,
                    commit: None,
                    author: None,
                    snippet: None,
                },
                ..create_minimal_finding()
            },
            CanonicalFinding {
                id: "F-002".into(),
                location: crate::find::FindingLocation {
                    file: unreachable_file,
                    line: None,
                    column: None,
                    commit: None,
                    author: None,
                    snippet: None,
                },
                ..create_minimal_finding()
            },
            CanonicalFinding {
                id: "F-003".into(),
                location: crate::find::FindingLocation {
                    file: PathBuf::from("."),
                    line: None,
                    column: None,
                    commit: None,
                    author: None,
                    snippet: None,
                },
                ..create_minimal_finding()
            },
        ];

        let reachable_set: HashSet<PathBuf> = [reachable_canon.clone()].into();
        let result = ReachabilityResult {
            enabled: true,
            reachable_files: reachable_set,
            entry_points: vec![reachable_canon.clone()],
            total_files_analyzed: 2,
            total_imports: 0,
            findings_reachable: 0,
            findings_unreachable: 0,
            findings_unknown: 3, // Before apply
        };

        apply_reachability(&mut findings, &result);

        assert_eq!(findings[0].reachable, Some(true));
        assert_eq!(findings[1].reachable, Some(false));
        assert_eq!(findings[2].reachable, None); // "." — no meaningful path
    }

    /// Helper to create a minimal CanonicalFinding for testing
    fn create_minimal_finding() -> CanonicalFinding {
        CanonicalFinding {
            id: String::new(),
            scanner: crate::find::ScannerType::Custom("test".into()),
            scanner_version: None,
            rule_id: String::new(),
            severity: crate::find::Severity::Info,
            confidence: crate::find::Confidence::Tentative,
            title: String::new(),
            description: String::new(),
            location: crate::find::FindingLocation {
                file: PathBuf::new(),
                line: None,
                column: None,
                commit: None,
                author: None,
                snippet: None,
            },
            cwe: None,
            cvss: None,
            remediation: None,
            fix_effort: None,
            evidence: None,
            tags: vec![],
            zt_pillars: vec![],
            cross_refs: vec![],
            grade: None,
            risk_score: None,
            reachable: None,
        }
    }

    #[test]
    fn test_reachability_config_defaults() {
        let cfg = ReachabilityConfig::default();
        assert!(!cfg.enabled);
        assert!(cfg.entry_points.is_empty());
        assert!(cfg.include_extensions.contains(&"rs".to_string()));
        assert!(cfg.exclude_dirs.contains(&".git".to_string()));
    }

    #[test]
    fn test_canonicalize_path_relative() {
        let tmp = create_temp_dir();
        let _file = write_file(tmp.path(), "test.rs", "fn test() {}");
        // Relative path from tmp
        let relative = PathBuf::from("test.rs");
        let canonical = canonicalize_path(&relative);
        // Should resolve to something
        assert!(!canonical.to_string_lossy().is_empty());
    }

    #[test]
    fn test_no_false_imports_for_std_extern() {
        // Rust `use std::...` and `use extern_crate::...` should NOT be resolved
        let tmp = create_temp_dir();
        let main_rs = write_file(
            tmp.path(),
            "src/main.rs",
            "use std::collections::HashMap;\nfn main() {}",
        );

        let imports = extract_imports(&main_rs, tmp.path());
        // Should not try to resolve std::collections::HashMap as a local file
        assert!(
            imports.is_empty(),
            "External imports like std::collections should not resolve"
        );
    }

    #[test]
    fn test_rust_super_imports() {
        let tmp = create_temp_dir();
        write_file(tmp.path(), "src/main.rs", "mod nested;\nfn main() {}");
        // -- src/main.rs uses `mod nested;` which means src/nested.rs or src/nested/mod.rs
        write_file(tmp.path(), "src/nested.rs", "pub fn foo() {}");
        let main_rs = tmp.path().join("src/main.rs");

        let imports = extract_imports(&main_rs, tmp.path());
        assert!(
            imports
                .iter()
                .any(|p| p.to_string_lossy().ends_with("nested.rs")),
            "Should find nested module"
        );
    }

    #[test]
    fn test_extract_js_tsx_extension() {
        let tmp = create_temp_dir();
        write_file(
            tmp.path(),
            "components/Button.tsx",
            "export const Button = () => null;",
        );
        let main_tsx = write_file(
            tmp.path(),
            "src/app.tsx",
            "import { Button } from '../components/Button';",
        );

        let imports = extract_imports(&main_tsx, tmp.path());
        assert!(
            imports
                .iter()
                .any(|p| p.to_string_lossy().ends_with("Button.tsx")),
            "Should resolve TSX imports"
        );
    }
}
