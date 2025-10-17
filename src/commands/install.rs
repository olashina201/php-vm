pub async fn run(version: String, from: Option<String>, ts: bool) -> anyhow::Result<()> {
    let layout = crate::core::dirs::ensure_layout()?;
    let cache_dir = layout.cache;
    let versions_dir = layout.versions;

    let url = if cfg!(windows) {
        let flavor = if ts { "ts" } else { "nts" };
        format!("https://windows.php.net/downloads/releases/php-{}-Win32-vs16-x64-{}.zip", version, flavor)
    } else {
        format!("https://www.php.net/distributions/php-{}.tar.xz", version)
    };

    let filename = url.split('/').last().unwrap_or("php-archive");
    let archive_path = cache_dir.join(filename);
    let _ = crate::io::downloader::download_with_resume(&url, &archive_path, None).await?;

    let extract_tmp = cache_dir.join(format!("extract-{}", version));
    if extract_tmp.exists() { fs_err::remove_dir_all(&extract_tmp)?; }
    if cfg!(unix) && matches!(from.as_deref(), Some("source")) {
        let jobs = crate::core::config::load_or_default(&layout.config)?.jobs.unwrap_or_else(num_cpus::get);
        if filename.ends_with(".tar.xz") {
            crate::io::archive::extract_tar_xz(&archive_path, &extract_tmp)?;
        } else if filename.ends_with(".tar.gz") {
            crate::io::archive::extract_tar_gz(&archive_path, &extract_tmp)?;
        } else {
            anyhow::bail!("source build requires a tar archive");
        }
        crate::platform::build_unix::build_from_source(&extract_tmp, &versions_dir.join(&version), jobs, &[]).await?;
        if extract_tmp.exists() { let _ = fs_err::remove_dir_all(&extract_tmp); }
        println!("built {} from source", version);
        return Ok(());
    } else {
        if filename.ends_with(".tar.xz") {
            crate::io::archive::extract_tar_xz(&archive_path, &extract_tmp)?;
        } else if filename.ends_with(".tar.gz") {
            crate::io::archive::extract_tar_gz(&archive_path, &extract_tmp)?;
        } else if filename.ends_with(".zip") {
            crate::io::archive::extract_zip(&archive_path, &extract_tmp)?;
        } else {
            anyhow::bail!("unsupported archive format: {}", filename);
        }
    }

    let mut root = None;
    for entry in fs_err::read_dir(&extract_tmp)? {
        let e = entry?;
        if e.file_type()?.is_dir() { root = Some(e.path()); break; }
    }
    let root = root.unwrap_or(extract_tmp.clone());
    let target = versions_dir.join(&version);
    if target.exists() { fs_err::remove_dir_all(&target)?; }
    fs_err::create_dir_all(&versions_dir)?;
    fs_err::rename(&root, &target)?;
    if extract_tmp.exists() { let _ = fs_err::remove_dir_all(&extract_tmp); }

    println!("installed {} -> {}", version, target.display());
    Ok(())
}


