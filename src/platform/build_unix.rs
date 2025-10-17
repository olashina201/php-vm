#[cfg(unix)]
pub async fn build_from_source(source_root: &std::path::Path, install_prefix: &std::path::Path, jobs: usize, extra_flags: &[String]) -> anyhow::Result<()> {
    use std::process::Command;
    use fs_err as fs;

    fs::create_dir_all(install_prefix)?;

    // Determine actual source directory (contains ./configure)
    let build_dir = {
        let direct = source_root.join("configure");
        if direct.exists() { source_root.to_path_buf() } else {
            let mut candidate = None;
            for entry in fs::read_dir(source_root)? {
                let e = entry?;
                if e.file_type()?.is_dir() {
                    let c = e.path().join("configure");
                    if c.exists() { candidate = Some(e.path()); break; }
                }
            }
            candidate.ok_or_else(|| anyhow::anyhow!("could not find ./configure in {:?}", source_root))?
        }
    };

    let mut flags: Vec<String> = vec![format!("--prefix={}", install_prefix.display())];
    flags.extend_from_slice(extra_flags);

    // ./configure
    let status = Command::new("sh").arg("-c").arg(format!("./configure {}", shell_join(&flags))).current_dir(&build_dir).status()?;
    if !status.success() { anyhow::bail!("configure failed"); }

    // make -j
    let status = Command::new("make").arg(format!("-j{}", jobs)).current_dir(&build_dir).status()?;
    if !status.success() { anyhow::bail!("make failed"); }

    // make install
    let status = Command::new("make").arg("install").current_dir(&build_dir).status()?;
    if !status.success() { anyhow::bail!("make install failed"); }

    Ok(())
}

#[cfg(unix)]
fn shell_join(args: &[String]) -> String {
    args.iter().map(|s| shell_escape::unix::escape(s.as_str().into()).to_string()).collect::<Vec<_>>().join(" ")
}

#[cfg(not(unix))]
pub async fn build_from_source(_source_root: &std::path::Path, _install_prefix: &std::path::Path, _jobs: usize, _extra_flags: &[String]) -> anyhow::Result<()> {
    anyhow::bail!("source build is only supported on Unix platforms")
}
