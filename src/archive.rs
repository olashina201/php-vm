use anyhow::Context;
use fs_err as fs;
use std::fs::File;
use std::path::{Path, PathBuf};

pub fn extract_tar_xz(archive_path: &Path, dest_dir: &Path) -> anyhow::Result<PathBuf> {
    fs::create_dir_all(dest_dir)?;
    let file = File::open(archive_path)?;
    let decompressor = xz2::read::XzDecoder::new(file);
    let mut archive = tar::Archive::new(decompressor);
    archive.unpack(dest_dir).with_context(|| format!("unpack tar.xz to {}", dest_dir.display()))?;
    Ok(dest_dir.to_path_buf())
}

pub fn extract_tar_gz(archive_path: &Path, dest_dir: &Path) -> anyhow::Result<PathBuf> {
    fs::create_dir_all(dest_dir)?;
    let file = File::open(archive_path)?;
    let decompressor = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(decompressor);
    archive.unpack(dest_dir).with_context(|| format!("unpack tar.gz to {}", dest_dir.display()))?;
    Ok(dest_dir.to_path_buf())
}

pub fn extract_zip(archive_path: &Path, dest_dir: &Path) -> anyhow::Result<PathBuf> {
    fs::create_dir_all(dest_dir)?;
    let file = File::open(archive_path)?;
    let mut zip = zip::ZipArchive::new(file)?;
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)?;
        let outpath = dest_dir.join(entry.mangled_name());
        if entry.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() { fs::create_dir_all(parent)?; }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut entry, &mut outfile)?;
            #[cfg(unix)] {
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = entry.unix_mode() { fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))?; }
            }
        }
    }
    Ok(dest_dir.to_path_buf())
}
