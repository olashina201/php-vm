pub async fn run(name: Option<String>, value: Option<String>, delete: bool) -> anyhow::Result<()> {
    let layout = crate::core::dirs::ensure_layout()?;
    let mut aliases = crate::core::aliases::load(&layout.root)?;
    match (name, value, delete) {
        (Some(n), _, true) => { aliases.0.remove(&n); println!("deleted alias {}", n); },
        (Some(n), Some(v), false) => { aliases.0.insert(n.clone(), v.clone()); println!("alias {} -> {}", n, v); },
        _ => {
            for (k, v) in aliases.0.iter() { println!("{}\t{}", k, v); }
        }
    }
    crate::core::aliases::save(&layout.root, &aliases)?;
    Ok(())
}


