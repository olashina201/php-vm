use anyhow::Context;
use fs_err as fs;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::header::{HeaderValue, RANGE};
use sha2::{Digest, Sha256};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use futures_util::StreamExt;

pub async fn download_with_resume(url: &str, dest: &Path, expected_sha256: Option<&str>) -> anyhow::Result<PathBuf> {
    let client = reqwest::Client::new();
    let tmp = dest.with_extension("part");

    // Existing size for resume
    let mut downloaded: u64 = if tmp.exists() { fs::metadata(&tmp)?.len() } else { 0 };

    let mut req = client.get(url);
    if downloaded > 0 {
        let range_header = format!("bytes={}-", downloaded);
        req = req.header(RANGE, HeaderValue::from_str(&range_header)?);
    }
    let resp = req.send().await?.error_for_status()?;

    let total = match resp.content_length() {
        Some(cl) if downloaded > 0 => cl + downloaded,
        Some(cl) => cl,
        None => 0,
    };

    let pb = ProgressBar::new(total);
    pb.set_style(ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})").unwrap());
    pb.set_position(downloaded);

    // Stream to file
    let mut file = fs::OpenOptions::new().create(true).append(true).open(&tmp)?;
    let mut stream = resp.bytes_stream();
    while let Some(chunk) = stream.next().await.transpose()? {
        file.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }
    pb.finish_and_clear();

    // Verify checksum if provided
    if let Some(expected) = expected_sha256 {
        let actual = sha256_file(&tmp)?;
        if !eq_sha256_hex(&actual, expected) {
            return Err(anyhow::anyhow!("checksum mismatch: expected {} got {}", expected, actual));
        }
    }

    fs::rename(&tmp, dest).with_context(|| format!("rename {} -> {}", tmp.display(), dest.display()))?;
    Ok(dest.to_path_buf())
}

fn sha256_file(path: &Path) -> anyhow::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}

fn eq_sha256_hex(a: &str, b: &str) -> bool {
    let na = a.trim().to_ascii_lowercase();
    let nb = b.trim().to_ascii_lowercase();
    na == nb
}
