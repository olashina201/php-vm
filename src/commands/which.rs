pub async fn run(program: Option<String>) -> anyhow::Result<()> {
    let layout = crate::core::dirs::ensure_layout()?;
    let program = program.unwrap_or_else(|| "php".into());
    if let Some(version) = crate::core::resolver::resolve_version(&layout, std::env::current_dir().ok().as_deref()) {
        let path = crate::platform::shims::which_executable(&layout, &version, &program)?;
        println!("{}", path.display());
        Ok(())
    } else {
        match which::which(&program) {
            Ok(p) => { println!("{}", p.display()); Ok(()) }
            Err(_) => anyhow::bail!("no version selected and '{}' not found on PATH", program),
        }
    }
}


