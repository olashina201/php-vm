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
    let _dirs = crate::dirs::ensure_layout()?;
    let cli = Cli::parse();
    match cli.command {
        Commands::ListRemote { major } => list_remote(major).await?,
        Commands::List => list_installed().await?,
        Commands::Install { version, from, ts } => install(version, from, ts).await?,
        Commands::Uninstall { version } => uninstall(version).await?,
        Commands::Use { version } => use_shell(version).await?,
        Commands::Global { target } => set_global(target).await?,
        Commands::Local { version } => set_local(version).await?,
        Commands::Which { program } => which(program).await?,
        Commands::Exec { version, cmd } => exec(version, cmd).await?,
        Commands::Alias { name, value, delete } => alias(name, value, delete).await?,
        Commands::Cache { cmd: CacheCommands::Prune } => cache_prune().await?,
        Commands::Doctor => doctor().await?,
        Commands::SelfUpdate => self_update().await?,
        Commands::Init { shell } => init(shell).await?,
    }
    Ok(())
}

async fn list_remote(major: Option<String>) -> anyhow::Result<()> {
    let manifest = crate::manifest::fetch_remote(major.clone()).await?;
    let mut versions = manifest.versions;
    if let Some(m) = major {
        versions.retain(|v| v.starts_with(&format!("{}.", m)) || v == &m);
    }
    for v in versions { println!("{}", v); }
    Ok(())
}
async fn list_installed() -> anyhow::Result<()> {
    let layout = crate::dirs::ensure_layout()?;
    let mut entries: Vec<_> = fs_err::read_dir(&layout.versions)?.filter_map(|e| e.ok()).filter(|e| e.file_type().ok().map(|t| t.is_dir()).unwrap_or(false)).map(|e| e.file_name().to_string_lossy().to_string()).collect();
    entries.sort();
    for v in entries { println!("{}", v); }
    let aliases = crate::aliases::load(&layout.root).unwrap_or_default();
    if !aliases.0.is_empty() {
        println!("\naliases:");
        for (k, v) in aliases.0.iter() { println!("{} -> {}", k, v); }
    }
    Ok(())
}
async fn install(version: String, from: Option<String>, ts: bool) -> anyhow::Result<()> {
    let layout = crate::dirs::ensure_layout()?;
    let cache_dir = layout.cache;
    let versions_dir = layout.versions;

    let url = if cfg!(windows) {
        let flavor = if ts { "ts" } else { "nts" };
        // Note: actual official naming includes compiler/toolset; this is a placeholder and may need adjustment per version
        format!("https://windows.php.net/downloads/releases/php-{}-Win32-vs16-x64-{}.zip", version, flavor)
    } else {
        format!("https://www.php.net/distributions/php-{}.tar.xz", version)
    };

    let filename = url.split('/').last().unwrap_or("php-archive");
    let archive_path = cache_dir.join(filename);
    let _ = crate::downloader::download_with_resume(&url, &archive_path, None).await?;

    // Extract to a temp dir under cache, then move into versions/<version>
    let extract_tmp = cache_dir.join(format!("extract-{}", version));
    if extract_tmp.exists() { fs_err::remove_dir_all(&extract_tmp)?; }
    if cfg!(unix) && matches!(from.as_deref(), Some("source")) {
        // Build from source
        let jobs = crate::config::load_or_default(&layout.config)?.jobs.unwrap_or_else(num_cpus::get);
        // If archive is not extracted, do it
        if filename.ends_with(".tar.xz") {
            crate::archive::extract_tar_xz(&archive_path, &extract_tmp)?;
        } else if filename.ends_with(".tar.gz") {
            crate::archive::extract_tar_gz(&archive_path, &extract_tmp)?;
        } else {
            anyhow::bail!("source build requires a tar archive");
        }
        crate::build_unix::build_from_source(&extract_tmp, &versions_dir.join(&version), jobs, &[]).await?;
        if extract_tmp.exists() { let _ = fs_err::remove_dir_all(&extract_tmp); }
        println!("built {} from source", version);
        return Ok(());
    } else {
        if filename.ends_with(".tar.xz") {
            crate::archive::extract_tar_xz(&archive_path, &extract_tmp)?;
        } else if filename.ends_with(".tar.gz") {
            crate::archive::extract_tar_gz(&archive_path, &extract_tmp)?;
        } else if filename.ends_with(".zip") {
            crate::archive::extract_zip(&archive_path, &extract_tmp)?;
        } else {
            anyhow::bail!("unsupported archive format: {}", filename);
        }
    }

    // Move the first directory level into versions/<version>
    let mut root = None;
    for entry in fs_err::read_dir(&extract_tmp)? {
        let e = entry?;
        if e.file_type()?.is_dir() { root = Some(e.path()); break; }
    }
    let root = root.unwrap_or(extract_tmp.clone());
    let target = versions_dir.join(&version);
    if target.exists() { fs_err::remove_dir_all(&target)?; }
    fs_err::create_dir_all(&versions_dir)?;
    fs_err::rename(&root, &target)?;
    if extract_tmp.exists() { let _ = fs_err::remove_dir_all(&extract_tmp); }

    println!("installed {} -> {}", version, target.display());
    Ok(())
}
async fn uninstall(version: String) -> anyhow::Result<()> {
    let layout = crate::dirs::ensure_layout()?;
    let target = layout.versions.join(&version);
    if target.exists() { fs_err::remove_dir_all(&target)?; println!("uninstalled {}", version); } else { println!("not installed: {}", version); }
    Ok(())
}
async fn use_shell(version: String) -> anyhow::Result<()> {
    // emit a shell eval snippet to set env for current shell
    println!("export PHPVM_VERSION=\"{}\"", version);
    Ok(())
}
async fn set_global(target: String) -> anyhow::Result<()> { let layout = crate::dirs::ensure_layout()?; crate::resolver::write_global(&layout, &target)?; println!("global set to {}", target); Ok(()) }
async fn set_local(version: String) -> anyhow::Result<()> { let cwd = std::env::current_dir()?; crate::resolver::write_local(&cwd, &version)?; println!("wrote .php-version {}", version); Ok(()) }
async fn which(program: Option<String>) -> anyhow::Result<()> {
    let layout = crate::dirs::ensure_layout()?;
    let program = program.unwrap_or_else(|| "php".into());
    if let Some(version) = crate::resolver::resolve_version(&layout, std::env::current_dir().ok().as_deref()) {
        let path = crate::shims::which_executable(&layout, &version, &program)?;
        println!("{}", path.display());
        Ok(())
    } else {
        // Fallback: show system PATH resolution if no phpvm version is selected
        match which::which(&program) {
            Ok(p) => { println!("{}", p.display()); Ok(()) }
            Err(_) => anyhow::bail!("no version selected and '{}' not found on PATH", program),
        }
    }
}
async fn exec(version: String, cmd: Vec<String>) -> anyhow::Result<()> {
    if cmd.is_empty() { anyhow::bail!("usage: phpvm exec <version> -- <cmd...>"); }
    let layout = crate::dirs::ensure_layout()?;
    // Prepend the selected version's bin directory to PATH
    let bin_dir = layout.versions.join(&version).join("bin");
    let mut command = std::process::Command::new(&cmd[0]);
    if cmd.len() > 1 { command.args(&cmd[1..]); }
    let path = std::env::var("PATH").unwrap_or_default();
    let prepend = if bin_dir.exists() { bin_dir } else { layout.versions.join(&version) };
    let new_path = format!("{}:{}", prepend.display(), path);
    command.env("PATH", new_path);
    command.env("PHPVM_VERSION", &version);
    let status = command.status()?;
    std::process::exit(status.code().unwrap_or(1));
}
async fn alias(name: Option<String>, value: Option<String>, delete: bool) -> anyhow::Result<()> {
    let layout = crate::dirs::ensure_layout()?;
    let mut aliases = crate::aliases::load(&layout.root)?;
    match (name, value, delete) {
        (Some(n), _, true) => { aliases.0.remove(&n); println!("deleted alias {}", n); },
        (Some(n), Some(v), false) => { aliases.0.insert(n.clone(), v.clone()); println!("alias {} -> {}", n, v); },
        _ => {
            for (k, v) in aliases.0.iter() { println!("{}\t{}", k, v); }
        }
    }
    crate::aliases::save(&layout.root, &aliases)?;
    Ok(())
}
async fn cache_prune() -> anyhow::Result<()> { let layout = crate::dirs::ensure_layout()?; if layout.cache.exists() { fs_err::remove_dir_all(&layout.cache)?; fs_err::create_dir_all(&layout.cache)?; } println!("cache pruned"); Ok(()) }
async fn doctor() -> anyhow::Result<()> {
    let layout = crate::dirs::ensure_layout()?;
    println!("root: {}", layout.root.display());
    println!("versions: {}", layout.versions.display());
    println!("shims: {}", layout.shims.display());
    println!("cache: {}", layout.cache.display());
    println!("Check: curl/reqwest OK; PATH contains shims? {}", std::env::var("PATH").unwrap_or_default().contains(&*layout.shims.to_string_lossy()));
    Ok(())
}
async fn self_update() -> anyhow::Result<()> { crate::self_update_cmd::run().await }
async fn init(shell: Option<String>) -> anyhow::Result<()> {
    let layout = crate::dirs::ensure_layout()?;
    crate::shims::ensure_shims(&layout)?;
    let shims = layout.shims.display().to_string();
    match shell.as_deref() {
        Some("fish") => println!("set -gx PATH {} $PATH;", shims),
        Some("zsh") | Some("bash") | Some("-") | None => println!("export PATH=\"{}:$PATH\"", shims),
        Some("powershell") | Some("pwsh") => {
            let module_path = layout.root.join("phpvm.psm1");
            let content = format!("$env:PATH=\"{};\" + $env:PATH\n", shims.replace(":", ";"));
            let _ = fs_err::write(&module_path, content);
            println!("# Add to PowerShell profile:\n# Import-Module '{}';", module_path.display());
            println!("$env:PATH=\"{};\" + $env:PATH", shims.replace(":", ";"));
        },
        Some(other) => println!("# unknown shell '{}', defaulting to POSIX\nexport PATH=\"{}:$PATH\"", other, shims),
    }
    Ok(())
}
