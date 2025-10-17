pub async fn run(major: Option<String>) -> anyhow::Result<()> {
    let manifest = crate::io::manifest::fetch_remote(major.clone()).await?;
    let mut versions = manifest.versions;
    if let Some(m) = major {
        versions.retain(|v| v.starts_with(&format!("{}.", m)) || v == &m);
    }
    for v in versions { println!("{}", v); }
    Ok(())
}


