// Context Drift Detection — File Discovery
// Regex patterns for claim extraction plus context file discovery and I/O helpers.
use super::types::ContextFileType;
use regex::Regex;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

pub(crate) fn dep_pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?i)(?:we\s+)?(?:use|run|depend\s+on|built\s+(?:with|on)|powered\s+by|via)\s+(.+?)(?:$|\.|,|\s+for\s+)"
        ).expect("invalid dep regex")
    })
}

pub(crate) fn version_pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?i)v?(\d+\.\d+(?:\.\d+)?(?:-(?:alpha|beta|rc|stable)\.?\d*)?)")
            .expect("invalid version regex")
    })
}

pub(crate) fn path_pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?i)(?:in|at|under|located\s+(?:in|at)|stored\s+in)\s+`?([\w/\.\-_]+)`?")
            .expect("invalid path regex")
    })
}

pub(crate) fn technology_keyword_pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?i)\b(PostgreSQL|MySQL|SQLite|Redis|MongoDB|DynamoDB|React|Vue|Angular|Svelte|Next\.js|Nuxt|Express|Fastify|Django|Flask|FastAPI|Rails|Spring|Laravel|Symfony|ASP\.NET|Node\.js|Deno|Bun|Rust|Go|Python|TypeScript|JavaScript|Kotlin|Swift|GraphQL|gRPC|REST|WebSocket|tRPC|Prisma|Drizzle|Sequelize|TypeORM|Docker|Kubernetes|AWS|GCP|Azure|Terraform|Ansible|JWT|OAuth|OIDC|SAML|Redis|Kafka|RabbitMQ|NATS|S3|CloudFront|Vercel|Netlify|Railway|Supabase|Firebase)\b").expect("invalid tech regex")
    })
}

/// Discover all agent context files in the project root
pub fn discover_context_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    let file_types = [
        ContextFileType::AgentsMd,
        ContextFileType::ClaudeMd,
        ContextFileType::CursorRules,
    ];

    for ft in &file_types {
        for name in ft.file_names() {
            let candidate = root.join(name);
            if candidate.exists() && candidate.is_file() {
                files.push(candidate);
            }
        }
    }

    files
}

/// Detect the context file type from its path
pub fn detect_file_type(path: &Path) -> Option<ContextFileType> {
    let file_name = path.file_name()?.to_str()?;
    let file_str = path.to_str().unwrap_or("");

    if file_str.contains(".cursor/rules") || file_str.ends_with(".cursor/rules") {
        return Some(ContextFileType::CursorRules);
    }

    match file_name {
        "AGENTS.md" | "AGENTS" => Some(ContextFileType::AgentsMd),
        "CLAUDE.md" | "CLAUDE" => Some(ContextFileType::ClaudeMd),
        _ => None,
    }
}

/// Read a file and return its content as a string, or None if unreadable
pub(crate) fn read_file(path: &Path) -> Option<String> {
    std::fs::read_to_string(path).ok()
}
