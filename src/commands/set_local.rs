pub async fn run(version: String) -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;
    crate::core::resolver::write_local(&cwd, &version)?;
    println!("wrote .php-version {}", version);
    Ok(())
}


