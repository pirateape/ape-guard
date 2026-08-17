// ApeGuard Configuration
// Loads and merges configuration from: defaults → file → environment variables → CLI flags
use crate::cli;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Scanner layers enabled (1=secrets, 2=SAST, 3=SCA, 4=container, 5=DAST)
    pub layers: Vec<u8>,

    /// Minimum severity filter
    pub severity: String,

    /// Scanner binary paths (auto-detected if not set)
    #[serde(default)]
    pub binaries: ScannerBinaries,

    /// Cache settings
    pub cache: CacheConfig,

    /// Report settings
    pub report: ReportConfig,

    /// LLM remediation settings (Ollama)
    #[serde(default)]
    pub llm: LlmConfig,

    /// Context drift detection settings (Layer 8)
    #[serde(default)]
    pub context_drift: ContextDriftConfig,

    /// False-positive suppression filters (Stage 3)
    #[serde(default)]
    pub filters: FilterConfig,

    /// Reachability analysis configuration (Phase 1.4)
    #[serde(default)]
    pub reachability: ReachabilityConfig,

    /// STRIDE threat model coverage analysis (Phase 2.1)
    #[serde(default)]
    pub stride: StrideConfig,

    /// Policy-as-Code settings (Phase 2.3)
    #[serde(default)]
    pub policy: crate::policy::PolicyConfig,

    /// Output directory
    pub output_dir: PathBuf,
}

/// False-positive suppression configuration.
/// All filters are opt-in — disabling them keeps findings that would otherwise be dropped.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterConfig {
    /// Exclude findings in test file paths (test/, *.test.*, *_test.go, etc.)
    pub exclude_test_files: bool,
    /// Exclude findings in vendor/third-party paths (node_modules/, vendor/, etc.)
    pub exclude_vendor: bool,
    /// Exclude findings in examples/sandbox/demo paths
    pub exclude_examples: bool,
    /// User-defined path patterns to exclude (case-insensitive substring match)
    pub exclude_paths: Vec<String>,
    /// Master switch for all path-based exclusions (test + vendor + examples + custom)
    pub exclude_paths_enabled: bool,
    /// Drop Info/Low findings in test files even if not path-excluded
    pub suppress_test_low_severity: bool,
    /// Require 2+ scanner confirmation (cross_refs) for Info/Low findings to survive
    pub require_cross_scanner_for_low: bool,
    /// Minimum confidence level: 0=Tentative, 1=Firm, 2=Certain. Findings below this are filtered.
    pub min_confidence: u8,
    /// Minimum severity as u8. Findings below this are filtered. None = no floor.
    pub min_severity: Option<u8>,
}

impl Default for FilterConfig {
    fn default() -> Self {
        Self {
            exclude_test_files: true,
            exclude_vendor: true,
            exclude_examples: false, // Often findings in examples are relevant
            exclude_paths: vec![],
            exclude_paths_enabled: true,
            suppress_test_low_severity: true,
            require_cross_scanner_for_low: false, // Off by default — too noisy for small teams
            min_confidence: 0,                    // Keep all confidence levels by default
            min_severity: None,                   // No severity floor by default
        }
    }
}

/// Reachability analysis configuration.
/// Determines which source files are transitively reachable from entry points.
/// Findings in unreachable (dead) code can be flagged as lower risk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReachabilityConfig {
    /// Master switch — reachability analysis is opt-in (default: false)
    pub enabled: bool,
    /// User-specified entry point file paths (relative to target directory)
    pub entry_points: Vec<String>,
    /// File extensions to include in analysis
    pub include_extensions: Vec<String>,
    /// Directories to exclude from analysis (relative names)
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

/// STRIDE threat model coverage analysis configuration (Phase 2.1).
/// Maps findings to the six STRIDE categories and identifies coverage gaps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrideConfig {
    /// Master switch — STRIDE analysis is opt-in (default: false)
    pub enabled: bool,
    /// Minimum coverage ratio (0.0–1.0) for a category to be considered "covered"
    pub coverage_threshold: f64,
}

impl Default for StrideConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            coverage_threshold: 0.05,
        }
    }
}

/// Context drift detection configuration (Layer 8)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextDriftConfig {
    /// Enable context drift detection by default
    pub enabled: bool,
    /// Whether to include unverifiable claims in results
    pub include_unknown: bool,
    /// Maximum drift findings to report
    pub max_findings: usize,
}

impl Default for ContextDriftConfig {
    fn default() -> Self {
        Self {
            enabled: false, // opt-in via --context-drift or config
            include_unknown: false,
            max_findings: 100,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScannerBinaries {
    pub gitleaks: Option<String>,
    pub trufflehog: Option<String>,
    pub semgrep: Option<String>,
    pub trivy: Option<String>,
    pub checkov: Option<String>,
    pub syft: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub enabled: bool,
    pub path: PathBuf,
    pub ttl_hours: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportConfig {
    pub formats: Vec<String>,
    pub types: Vec<String>,
}

/// LLM remediation settings — mirrors llm::LlmConfig to keep config serialisable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// Ollama endpoint (default: http://localhost:11434)
    pub endpoint: String,
    /// Model name (default: codellama)
    pub model: String,
    /// Whether LLM enhancement is enabled
    pub enabled: bool,
}

impl Default for LlmConfig {
    fn default() -> Self {
        LlmConfig {
            endpoint: "http://localhost:11434".to_string(),
            model: "codellama".to_string(),
            enabled: true,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            layers: vec![1, 2, 3, 4],
            severity: "all".to_string(),
            binaries: ScannerBinaries {
                gitleaks: None,
                trufflehog: None,
                semgrep: None,
                trivy: None,
                checkov: None,
                syft: None,
            },
            cache: CacheConfig {
                enabled: true,
                path: PathBuf::from(".apeguard/cache"),
                ttl_hours: 24,
            },
            report: ReportConfig {
                formats: vec!["md".to_string()],
                types: vec![
                    "tech".to_string(),
                    "exec".to_string(),
                    "roadmap".to_string(),
                ],
            },
            llm: LlmConfig::default(),
            context_drift: ContextDriftConfig::default(),
            filters: FilterConfig::default(),
            reachability: ReachabilityConfig::default(),
            stride: StrideConfig::default(),
            policy: crate::policy::PolicyConfig::default(),
            output_dir: PathBuf::from(".apeguard/reports"),
        }
    }
}

/// Load configuration by merging: defaults → file → env → CLI args
pub fn load(args: &cli::Args) -> anyhow::Result<Config> {
    let mut cfg = Config::default();

    // 1. Load from config file if specified
    if let Some(config_path) = &args.config {
        let path = PathBuf::from(config_path);
        if path.exists() {
            let contents = std::fs::read_to_string(&path)?;
            let file_cfg: Config = serde_yaml::from_str(&contents)?;
            merge(&mut cfg, file_cfg);
        }
    } else {
        // Auto-discover .apeguard.yaml in current directory
        let auto_paths = [
            PathBuf::from(".apeguard.yaml"),
            PathBuf::from(".apeguard/config.yaml"),
            PathBuf::from("apeguard.yaml"),
        ];
        for p in &auto_paths {
            if p.exists() {
                let contents = std::fs::read_to_string(p)?;
                let file_cfg: Config = serde_yaml::from_str(&contents)?;
                merge(&mut cfg, file_cfg);
                break;
            }
        }
    }

    // 2. Override from env vars
    if let Ok(val) = std::env::var("APEGUARD_LAYERS") {
        cfg.layers = val
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
    }
    if let Ok(val) = std::env::var("APEGUARD_SEVERITY") {
        cfg.severity = val;
    }
    if let Ok(val) = std::env::var("APEGUARD_OUTPUT_DIR") {
        cfg.output_dir = PathBuf::from(val);
    }

    // 3. Override from CLI args
    if args.command.is_scan() {
        if let cli::Command::Scan {
            ref layers,
            ref severity,
            ref output_dir,
            ..
        } = args.command
        {
            if !layers.is_empty() {
                cfg.layers = layers.clone();
            }
            if let cli::SeverityFilter::All = severity {
                // keep default
            } else {
                cfg.severity = format!("{:?}", severity).to_lowercase();
            }
            cfg.output_dir = PathBuf::from(output_dir);
        }
    }

    Ok(cfg)
}

/// Merge a partial config into the current one (non-destructive overlay).
fn merge(base: &mut Config, overlay: Config) {
    // Only override layers when the overlay provides explicitly (non-empty vector)
    if !overlay.layers.is_empty() {
        base.layers = overlay.layers;
    }
    // Override severity only when explicitly set (non-default "all")
    if overlay.severity != "all" {
        base.severity = overlay.severity;
    }
    // Scanner binary paths — overlay takes precedence when Some
    if overlay.binaries.gitleaks.is_some() {
        base.binaries.gitleaks = overlay.binaries.gitleaks;
    }
    if overlay.binaries.trufflehog.is_some() {
        base.binaries.trufflehog = overlay.binaries.trufflehog;
    }
    if overlay.binaries.semgrep.is_some() {
        base.binaries.semgrep = overlay.binaries.semgrep;
    }
    if overlay.binaries.trivy.is_some() {
        base.binaries.trivy = overlay.binaries.trivy;
    }
    if overlay.binaries.checkov.is_some() {
        base.binaries.checkov = overlay.binaries.checkov;
    }
    if overlay.binaries.syft.is_some() {
        base.binaries.syft = overlay.binaries.syft;
    }
    // Cache settings
    base.cache = overlay.cache;
    // Report settings (formats/types always taken from overlay)
    base.report = overlay.report;
    // LLM settings
    base.llm = overlay.llm;
    // Context drift settings
    base.context_drift = overlay.context_drift;
    // Filters
    base.filters = overlay.filters;
    // Reachability
    base.reachability = overlay.reachability;
    // STRIDE
    base.stride = overlay.stride;
    // Policy
    base.policy = overlay.policy;
    // Output directory
    base.output_dir = overlay.output_dir;
}

/// Generate a default .apeguard.yaml config file
pub fn generate_init(path: Option<String>, _template: cli::InitTemplate) -> anyhow::Result<()> {
    let target = path.map(PathBuf::from).unwrap_or_default();
    let config_path = if target.as_os_str().is_empty() {
        PathBuf::from(".apeguard.yaml")
    } else {
        target.join(".apeguard.yaml") // works for both file and dir targets
    };

    if config_path.exists() {
        anyhow::bail!("Config file already exists at: {}", config_path.display());
    }

    let default_cfg = Config::default();
    let yaml = serde_yaml::to_string(&default_cfg)?;

    // Add helpful comments
    let commented = format!(
        "# ApeGuard Configuration\n\
         # Generated by `apeguard init`\n\
         # See https://github.com/pirateape/ape-guard for full docs\n\
         \n\
         {yaml}\n\
         # Scanner layers:\n\
         #   1 = Secrets (Gitleaks)\n\
         #   2 = SAST (Semgrep)\n\
         #   3 = SCA (Trivy vulns)\n\
         #   4 = Container (Trivy image)\n\
         #   5 = DAST (Nuclei/Zap)\n"
    );

    std::fs::write(&config_path, commented)?;
    tracing::info!("Created config: {}", config_path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let cfg = Config::default();
        assert_eq!(cfg.layers, vec![1, 2, 3, 4]);
        assert_eq!(cfg.severity, "all");
        assert!(cfg.cache.enabled);
        assert_eq!(cfg.report.formats, vec!["md"]);
        assert_eq!(cfg.report.types.len(), 3);
    }

    #[test]
    fn test_merge_overlay() {
        let mut base = Config::default();
        let overlay = Config {
            layers: vec![1],
            severity: "high".into(),
            binaries: ScannerBinaries {
                gitleaks: Some("/usr/local/bin/gitleaks".into()),
                trufflehog: None,
                semgrep: None,
                trivy: None,
                checkov: None,
                syft: None,
            },
            ..Config::default()
        };
        merge(&mut base, overlay);
        assert_eq!(base.layers, vec![1]);
        assert_eq!(base.severity, "high");
        assert_eq!(
            base.binaries.gitleaks,
            Some("/usr/local/bin/gitleaks".into())
        );
        assert!(base.binaries.semgrep.is_none());
    }

    #[test]
    fn test_merge_empty_layers_does_not_override() {
        let mut base = Config::default();
        assert_eq!(base.layers, vec![1, 2, 3, 4]);
        let overlay = Config {
            layers: vec![],
            ..Config::default()
        };
        merge(&mut base, overlay);
        // Should keep original layers when overlay layers are empty
        assert_eq!(base.layers, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_merge_report_formats() {
        let mut base = Config::default();
        let overlay = Config {
            report: ReportConfig {
                formats: vec!["html".into()],
                types: vec!["tech".into()],
            },
            ..Config::default()
        };
        merge(&mut base, overlay);
        assert_eq!(base.report.formats, vec!["html"]);
    }

    #[test]
    fn test_config_yaml_roundtrip() {
        let cfg = Config::default();
        let yaml = serde_yaml::to_string(&cfg).expect("config test: failed to serialize YAML");
        let parsed: Config =
            serde_yaml::from_str(&yaml).expect("config test: failed to deserialize YAML");
        assert_eq!(cfg.layers, parsed.layers);
        assert_eq!(cfg.severity, parsed.severity);
        assert_eq!(cfg.cache.enabled, parsed.cache.enabled);
    }

    #[test]
    fn test_config_yaml_custom_values() {
        let yaml = r#"
layers:
  - 1
  - 5
severity: "critical"
cache:
  enabled: false
  path: "/tmp/cache"
  ttl_hours: 48
"#;
        let cfg: Config = serde_yaml::from_str(yaml).expect("config test: failed to parse YAML");
        assert_eq!(cfg.layers, vec![1, 5]);
        assert_eq!(cfg.severity, "critical");
        assert!(!cfg.cache.enabled);
        assert_eq!(cfg.cache.ttl_hours, 48);
    }

    #[test]
    fn test_generate_init_fails_if_exists() {
        let tmpdir = tempfile::tempdir().expect("failed to create temp dir for config test");
        let config_path = tmpdir.path().join(".apeguard.yaml");
        std::fs::write(&config_path, "existing: true")
            .expect("config test: failed to write config file");

        let result = generate_init(
            Some(
                tmpdir
                    .path()
                    .to_str()
                    .expect("config test: temp path is not valid UTF-8")
                    .to_string(),
            ),
            crate::cli::InitTemplate::Default,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));
    }

    #[test]
    fn test_scanner_binaries_defaults() {
        let bins = ScannerBinaries {
            gitleaks: None,
            trufflehog: None,
            semgrep: None,
            trivy: None,
            checkov: None,
            syft: None,
        };
        assert!(bins.gitleaks.is_none());
        assert!(bins.trufflehog.is_none());
        assert!(bins.semgrep.is_none());
        assert!(bins.trivy.is_none());
        assert!(bins.checkov.is_none());
        assert!(bins.syft.is_none());
    }
}
