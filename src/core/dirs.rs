use anyhow::Context;
use directories::BaseDirs;
use fs_err as fs;
use std::path::{Path, PathBuf};

pub struct Layout {
    pub root: PathBuf,
    pub versions: PathBuf,
    pub shims: PathBuf,
    pub cache: PathBuf,
    #[allow(dead_code)]
    pub manifests: PathBuf,
    pub config: PathBuf,
}

pub fn ensure_layout() -> anyhow::Result<Layout> {
    let base = BaseDirs::new().context("cannot get base dirs")?;
    let root = base.home_dir().join(".phpvm");
    let versions = root.join("versions");
    let shims = root.join("shims");
    let cache = root.join("cache");
    let manifests = root.join("manifests");
    let config = root.join("config.toml");

    for d in [&root, &versions, &shims, &cache, &manifests] { fs::create_dir_all(d).with_context(|| format!("create dir: {}", d.display()))?; }

    Ok(Layout { root, versions, shims, cache, manifests, config })
}

#[allow(dead_code)]
pub fn path_in<P: AsRef<Path>>(base: &Path, child: P) -> PathBuf { base.join(child) }
