// ApeGuard CLI Argument Parsing
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "apeguard", version, about = "Security posture assessment — one command, three reports, Zero Trust mapped")]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,

    /// Path to config file
    #[arg(short, long, global = true)]
    pub config: Option<String>,

    /// Log level (trace, debug, info, warn, error)
    #[arg(long, global = true, default_value = "info")]
    pub log_level: String,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Suppress all output except results
    #[arg(short, long, global = true)]
    pub quiet: bool,
}

#[derive(Subcommand)]
pub enum Command {
    /// Run a full security assessment
    Scan {
        /// Target directory or repository
        path: Option<String>,

        /// Scanner layers to run (1=secrets, 2=SAST, 3=SCA, 4=container, 5=DAST)
        #[arg(long, value_delimiter = ',', default_value = "1,2,3,4")]
        layers: Vec<u8>,

        /// Web target URL (enables DAST)
        #[arg(long)]
        web: Option<String>,

        /// Minimum severity (info, low, medium, high, critical)
        #[arg(long, default_value = "all")]
        severity: SeverityFilter,

        /// Force full re-scan
        #[arg(long)]
        no_cache: bool,

        /// Output formats
        #[arg(long, value_delimiter = ',', default_value = "md")]
        format: Vec<OutputFormat>,

        /// Report types
        #[arg(long, value_delimiter = ',', default_value = "tech,exec,roadmap")]
        reports: Vec<ReportType>,

        /// Exit code behavior
        #[arg(long, default_value = "never")]
        fail_on: FailOnThreshold,

        /// Output directory
        #[arg(long, default_value = ".apeguard/reports")]
        output_dir: String,
    },

    /// Regenerate reports from cached scan
    Report {
        /// Target directory
        path: Option<String>,

        /// Snapshot ID (default: latest)
        #[arg(long)]
        snapshot: Option<String>,

        /// Output formats
        #[arg(long, value_delimiter = ',', default_value = "md")]
        format: Vec<OutputFormat>,

        /// Report types
        #[arg(long, value_delimiter = ',', default_value = "tech,exec,roadmap")]
        reports: Vec<ReportType>,

        /// Output directory
        #[arg(long, default_value = ".apeguard/reports")]
        output_dir: String,
    },

    /// Compare two scan snapshots
    Compare {
        /// First snapshot ID or path
        a: String,

        /// Second snapshot ID or path
        b: String,

        /// Output format
        #[arg(long, default_value = "text")]
        format: CompareFormat,
    },

    /// Create .apeguard.yaml configuration
    Init {
        /// Target directory
        path: Option<String>,

        /// Template preset
        #[arg(long, default_value = "default")]
        template: InitTemplate,
    },

    /// Show or validate configuration
    Config {
        #[command(subcommand)]
        subcommand: Option<ConfigSubcommand>,
    },

    /// Show version and dependency status
    Version,

    /// Generate shell completions
    Completions {
        /// Shell type
        shell: clap_complete::Shell,
    },
}

#[derive(Clone, ValueEnum)]
pub enum SeverityFilter {
    All,
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    Md,
    Json,
    Sarif,
    Html,
    Pdf,
}

#[derive(Clone, ValueEnum)]
pub enum ReportType {
    Tech,
    Exec,
    Roadmap,
}

#[derive(Clone, ValueEnum)]
pub enum FailOnThreshold {
    Never,
    High,
    Critical,
}

#[derive(Clone, ValueEnum)]
pub enum CompareFormat {
    Text,
    Json,
    Html,
}

#[derive(Clone, ValueEnum)]
pub enum InitTemplate {
    Default,
    Ci,
    Minimal,
}

#[derive(Subcommand)]
pub enum ConfigSubcommand {
    /// Validate configuration
    Validate,
    /// Show config file search paths
    Paths,
}

pub fn parse() -> Args {
    Args::parse()
}

pub fn generate_completions(shell: clap_complete::Shell) {
    use clap::CommandFactory;
    let mut cmd = Args::command();
    let name = cmd.get_name().to_string();
    clap_complete::generate(shell, &mut cmd, name, &mut std::io::stdout());
}
