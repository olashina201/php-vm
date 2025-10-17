pub async fn run() -> anyhow::Result<()> {
    let layout = crate::core::dirs::ensure_layout()?;
    if layout.cache.exists() { fs_err::remove_dir_all(&layout.cache)?; fs_err::create_dir_all(&layout.cache)?; }
    println!("cache pruned");
    Ok(())
}


