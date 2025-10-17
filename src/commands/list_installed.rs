pub async fn run() -> anyhow::Result<()> {
    let layout = crate::core::dirs::ensure_layout()?;
    let mut entries: Vec<_> = fs_err::read_dir(&layout.versions)?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().ok().map(|t| t.is_dir()).unwrap_or(false))
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    entries.sort();
    for v in entries { println!("{}", v); }
    let aliases = crate::core::aliases::load(&layout.root).unwrap_or_default();
    if !aliases.0.is_empty() {
        println!("\naliases:");
        for (k, v) in aliases.0.iter() { println!("{} -> {}", k, v); }
    }
    Ok(())
}


