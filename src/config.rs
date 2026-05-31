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

    /// Output directory
    pub output_dir: PathBuf,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScannerBinaries {
    pub gitleaks: Option<String>,
    pub semgrep: Option<String>,
    pub trivy: Option<String>,
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
                semgrep: None,
                trivy: None,
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

/// Merge a partial config into the current one (non-destructive overlay)
fn merge(base: &mut Config, overlay: Config) {
    if !overlay.layers.is_empty() {
        base.layers = overlay.layers;
    }
    if overlay.severity != "all" {
        base.severity = overlay.severity;
    }
    if overlay.binaries.gitleaks.is_some() {
        base.binaries.gitleaks = overlay.binaries.gitleaks;
    }
    if overlay.binaries.semgrep.is_some() {
        base.binaries.semgrep = overlay.binaries.semgrep;
    }
    if overlay.binaries.trivy.is_some() {
        base.binaries.trivy = overlay.binaries.trivy;
    }
    if overlay.cache.enabled != base.cache.enabled {
        base.cache.enabled = overlay.cache.enabled;
    }
    if overlay.cache.path.as_path() != std::path::Path::new(".apeguard/cache") {
        base.cache.path = overlay.cache.path;
    }
    if overlay.cache.ttl_hours != 24 {
        base.cache.ttl_hours = overlay.cache.ttl_hours;
    }
    if overlay.report.formats != vec!["md"] {
        base.report.formats = overlay.report.formats;
    }
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
                semgrep: None,
                trivy: None,
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
        let yaml = serde_yaml::to_string(&cfg).unwrap();
        let parsed: Config = serde_yaml::from_str(&yaml).unwrap();
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
        let cfg: Config = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(cfg.layers, vec![1, 5]);
        assert_eq!(cfg.severity, "critical");
        assert!(!cfg.cache.enabled);
        assert_eq!(cfg.cache.ttl_hours, 48);
    }

    #[test]
    fn test_generate_init_fails_if_exists() {
        let tmpdir = tempfile::tempdir().unwrap();
        let config_path = tmpdir.path().join(".apeguard.yaml");
        std::fs::write(&config_path, "existing: true").unwrap();

        let result = generate_init(
            Some(tmpdir.path().to_str().unwrap().to_string()),
            crate::cli::InitTemplate::Default,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));
    }

    #[test]
    fn test_scanner_binaries_defaults() {
        let bins = ScannerBinaries {
            gitleaks: None,
            semgrep: None,
            trivy: None,
        };
        assert!(bins.gitleaks.is_none());
        assert!(bins.semgrep.is_none());
        assert!(bins.trivy.is_none());
    }
}
