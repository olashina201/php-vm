pub async fn run(target: String) -> anyhow::Result<()> {
    let layout = crate::core::dirs::ensure_layout()?;
    crate::core::resolver::write_global(&layout, &target)?;
    println!("global set to {}", target);
    Ok(())
}


