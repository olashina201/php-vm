use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteManifest {
    pub versions: Vec<String>,
}

#[derive(Deserialize)]
struct GithubTag { name: String }

pub async fn fetch_remote(_maybe_major: Option<String>) -> anyhow::Result<RemoteManifest> {
    // Fetch tags from GitHub php/php-src and parse versions from tag names (e.g., "php-8.3.11").
    let client = reqwest::Client::new();
    let mut versions: Vec<String> = Vec::new();
    let mut page = 1u32;
    // Fetch up to 5 pages x 100 tags = 500 tags (adjust as needed)
    while page <= 5 {
        let url = format!("https://api.github.com/repos/php/php-src/tags?per_page=100&page={}", page);
        let resp = client
            .get(&url)
            .header("User-Agent", "phpvm/0.1 (github.com/example/phpvm)")
            .send()
            .await?
            .error_for_status()?;
        let tags: Vec<GithubTag> = resp.json().await?;
        if tags.is_empty() { break; }
        for t in tags {
            if let Some(v) = parse_tag_to_version(&t.name) { versions.push(v); }
        }
        page += 1;
    }
    // Unique + sort desc (lexicographically is acceptable for dotted versions for now)
    versions.sort();
    versions.dedup();
    versions.reverse();
    Ok(RemoteManifest { versions })
}

fn parse_tag_to_version(tag: &str) -> Option<String> {
    // Accept patterns like "php-8.3.11", "php-8.2.23RC1" (strip suffix), or "php-8.1.29"
    // Find first digit start
    let bytes = tag.as_bytes();
    let mut start = None;
    for (i, b) in bytes.iter().enumerate() {
        if b.is_ascii_digit() { start = Some(i); break; }
    }
    let start = start?;
    let remainder = &tag[start..];
    // Take digits/dots until non-matching char
    let mut out = String::new();
    for ch in remainder.chars() {
        if ch.is_ascii_digit() || ch == '.' { out.push(ch); } else { break; }
    }
    if out.split('.').count() >= 2 { Some(out) } else { None }
}
