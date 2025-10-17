pub async fn run() -> anyhow::Result<()> {
    let status = tokio::task::spawn_blocking(|| -> anyhow::Result<String> {
        let mut updater = self_update::backends::github::Update::configure();
        let updater = updater
            .repo_owner("your-github-user-or-org")
            .repo_name("phpvm")
            .bin_name("phpvm")
            .show_download_progress(true)
            .current_version(env!("CARGO_PKG_VERSION"))
            .build()?;
        match updater.update() {
            Ok(status) => Ok(format!("{}", status.version())),
            Err(e) => Err(anyhow::anyhow!(e)),
        }
    }).await?;

    match status {
        Ok(new_ver) => {
            println!("updated to {}", new_ver);
            Ok(())
        }
        Err(err) => {
            eprintln!("self-update: {}", err);
            eprintln!("Tip: ensure releases exist on GitHub with assets named 'phpvm-<target>'.");
            Ok(())
        }
    }
}
