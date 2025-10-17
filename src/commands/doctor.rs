pub async fn run() -> anyhow::Result<()> {
    let layout = crate::core::dirs::ensure_layout()?;
    println!("root: {}", layout.root.display());
    println!("versions: {}", layout.versions.display());
    println!("shims: {}", layout.shims.display());
    println!("cache: {}", layout.cache.display());
    println!("Check: curl/reqwest OK; PATH contains shims? {}", std::env::var("PATH").unwrap_or_default().contains(&*layout.shims.to_string_lossy()));
    Ok(())
}


