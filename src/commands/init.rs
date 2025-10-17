pub async fn run(shell: Option<String>) -> anyhow::Result<()> {
    let layout = crate::core::dirs::ensure_layout()?;
    crate::platform::shims::ensure_shims(&layout)?;
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


