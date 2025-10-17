use clap::{ArgAction, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "phpvm", version, about = "PHP version manager (Rust)")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "list-remote")]
    ListRemote { #[arg(long)] major: Option<String> },
    #[command(name = "list")]
    List,
    #[command(name = "install")]
    Install { version: String, #[arg(long)] from: Option<String>, #[arg(long, help = "Windows: use thread-safe (ts) build")] ts: bool },
    #[command(name = "uninstall")]
    Uninstall { version: String },
    #[command(name = "use")]
    Use { version: String },
    #[command(name = "global")]
    Global { target: String },
    #[command(name = "local")]
    Local { version: String },
    #[command(name = "which")]
    Which { program: Option<String> },
    #[command(name = "exec")]
    Exec { version: String, #[arg(trailing_var_arg=true)] cmd: Vec<String> },
    #[command(name = "alias")]
    Alias { name: Option<String>, value: Option<String>, #[arg(long, action=ArgAction::SetTrue)] delete: bool },
    #[command(name = "cache")]
    Cache { #[command(subcommand)] cmd: CacheCommands },
    #[command(name = "doctor")]
    Doctor,
    #[command(name = "self-update")]
    SelfUpdate,
    #[command(name = "init")]
    Init { #[arg(long)] shell: Option<String> },
}

#[derive(Subcommand)]
pub enum CacheCommands {
    #[command(name = "prune")]
    Prune,
}

pub async fn run() -> anyhow::Result<()> {
    let _dirs = crate::core::dirs::ensure_layout()?;
    let cli = Cli::parse();
    match cli.command {
        Commands::ListRemote { major } => crate::commands::list_remote::run(major).await?,
        Commands::List => crate::commands::list_installed::run().await?,
        Commands::Install { version, from, ts } => crate::commands::install::run(version, from, ts).await?,
        Commands::Uninstall { version } => crate::commands::uninstall::run(version).await?,
        Commands::Use { version } => crate::commands::use_shell::run(version).await?,
        Commands::Global { target } => crate::commands::set_global::run(target).await?,
        Commands::Local { version } => crate::commands::set_local::run(version).await?,
        Commands::Which { program } => crate::commands::which::run(program).await?,
        Commands::Exec { version, cmd } => crate::commands::exec::run(version, cmd).await?,
        Commands::Alias { name, value, delete } => crate::commands::alias::run(name, value, delete).await?,
        Commands::Cache { cmd: CacheCommands::Prune } => crate::commands::cache_prune::run().await?,
        Commands::Doctor => crate::commands::doctor::run().await?,
        Commands::SelfUpdate => crate::commands::self_update::run().await?,
        Commands::Init { shell } => crate::commands::init::run(shell).await?,
    }
    Ok(())
}
