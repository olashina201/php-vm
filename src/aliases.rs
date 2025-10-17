use fs_err as fs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Aliases(pub HashMap<String, String>);

pub fn load(root: &std::path::Path) -> anyhow::Result<Aliases> {
    let path = root.join("aliases.toml");
    if path.exists() {
        let text = fs::read_to_string(path)?;
        let map: Aliases = toml::from_str(&text)?;
        Ok(map)
    } else {
        Ok(Aliases::default())
    }
}

pub fn save(root: &std::path::Path, aliases: &Aliases) -> anyhow::Result<()> {
    let path = root.join("aliases.toml");
    let data = toml::to_string_pretty(aliases)?;
    fs::write(path, data)?;
    Ok(())
}

#[allow(dead_code)]
pub fn resolve_alias(aliases: &Aliases, name: &str) -> Option<String> { aliases.0.get(name).cloned() }
