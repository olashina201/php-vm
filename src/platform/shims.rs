use fs_err as fs;
use std::path::{Path, PathBuf};

pub fn ensure_shims(layout: &crate::core::dirs::Layout) -> anyhow::Result<()> {
    fs::create_dir_all(&layout.shims)?;
    for name in ["php", "pecl", "pear", "composer"] {
        install_shim(&layout.shims, name)?;
    }
    Ok(())
}

fn install_shim(dir: &Path, name: &str) -> anyhow::Result<()> {
    #[cfg(windows)]
    {
        // Write a .cmd shim
        let path = dir.join(format!("{}.cmd", name));
        let content = format!(
            "@echo off\r\n".
            to_owned() + "setlocal enabledelayedexpansion\r\n" +
            "phpvm which " + name + " >nul 2>nul\r\n" +
            "for /f \"usebackq tokens=*\" %%i in (`phpvm which " + name + "`) do set TARGET=%%i\r\n" +
            "%TARGET% %*\r\n"
        );
        fs::write(path, content)?;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let path = dir.join(name);
        let content = format!("#!/usr/bin/env sh\nexec phpvm which {} >/dev/null 2>&1; TARGET=$(phpvm which {}); exec \"$TARGET\" \"$@\"\n", name, name);
        fs::write(&path, content.as_bytes())?;
        fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))?;
    }
    Ok(())
}

pub fn which_executable(layout: &crate::core::dirs::Layout, version: &str, program: &str) -> anyhow::Result<PathBuf> {
    let candidate = layout.versions.join(version).join("bin").join(program);
    if candidate.exists() { return Ok(candidate); }
    // Windows layout may differ, try root
    let candidate = layout.versions.join(version).join(program);
    if candidate.exists() { return Ok(candidate); }
    anyhow::bail!("program not found for {}: {}", version, program)
}
