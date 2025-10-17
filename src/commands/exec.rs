pub async fn run(version: String, cmd: Vec<String>) -> anyhow::Result<()> {
    if cmd.is_empty() { anyhow::bail!("usage: phpvm exec <version> -- <cmd...>"); }
    let layout = crate::core::dirs::ensure_layout()?;
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


