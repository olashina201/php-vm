pub async fn run(version: String) -> anyhow::Result<()> {
    let layout = crate::core::dirs::ensure_layout()?;
    let target = layout.versions.join(&version);
    if target.exists() { fs_err::remove_dir_all(&target)?; println!("uninstalled {}", version); } else { println!("not installed: {}", version); }
    Ok(())
}


