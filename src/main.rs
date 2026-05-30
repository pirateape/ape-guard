// ApeGuard CLI — Security Posture Assessment
// One command. Three reports. Zero Trust mapped.
//
// Architecture: docs/03-Projects/ApeGuard/ApeGuard_Architecture.md

mod cli;
mod config;
mod scanner;
mod find;
mod normalize;
mod dedup;
mod cache;
mod report;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Parse CLI args
    let args = cli::parse();

    // Load config (merge defaults + file + env + flags)
    let cfg = config::load(&args)?;

    match args.command {
        cli::Command::Scan { path, layers, severity, output_dir, .. } => {
            run_scan(path, layers, severity, output_dir, &cfg).await?;
        }
        cli::Command::Report { path, snapshot, output_dir, .. } => {
            run_report(path, snapshot, output_dir, &cfg).await?;
        }
        cli::Command::Compare { a, b, .. } => {
            run_compare(a, b).await?;
        }
        cli::Command::Init { path, template } => {
            config::generate_init(path, template)?;
        }
        cli::Command::Config { subcommand } => {
            handle_config(subcommand, &cfg)?;
        }
        cli::Command::Version => {
            print_version().await?;
        }
        cli::Command::Completions { shell } => {
            cli::generate_completions(shell);
        }
    }

    Ok(())
}
