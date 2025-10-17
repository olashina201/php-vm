pub async fn run(version: String) -> anyhow::Result<()> {
    println!("export PHPVM_VERSION=\"{}\"", version);
    Ok(())
}


