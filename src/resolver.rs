use fs_err as fs;
use std::env;
use std::path::Path;

pub fn resolve_version(layout: &crate::dirs::Layout, cwd: Option<&Path>) -> Option<String> {
    if let Ok(v) = env::var("PHPVM_VERSION") { if !v.trim().is_empty() { return Some(v); } }
    if let Some(dir) = cwd { if let Some(v) = find_php_version_file(dir) { return Some(v); } }
    read_global(&layout.root)
}

fn find_php_version_file(start: &Path) -> Option<String> {
    let mut dir = Some(start);
    while let Some(d) = dir {
        let path = d.join(".php-version");
        if path.exists() { return fs::read_to_string(path).ok().map(|s| s.trim().to_string()); }
        dir = d.parent();
    }
    None
}

fn read_global(root: &Path) -> Option<String> {
    let p = root.join("global");
    if p.exists() { fs::read_to_string(p).ok().map(|s| s.trim().to_string()) } else { None }
}

pub fn write_global(layout: &crate::dirs::Layout, version: &str) -> anyhow::Result<()> {
    let p = layout.root.join("global");
    fs::write(p, version.as_bytes())?;
    Ok(())
}

pub fn write_local(cwd: &Path, version: &str) -> anyhow::Result<()> {
    let p = cwd.join(".php-version");
    fs::write(p, version.as_bytes())?;
    Ok(())
}
