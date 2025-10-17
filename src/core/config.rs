use fs_err as fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub configure_flags: Option<Vec<String>>, // default ./configure flags
    pub jobs: Option<usize>,                  // make -j
    pub default_extensions: Option<Vec<String>>, // pecl to auto-install/enable later
}

impl Default for Config {
    fn default() -> Self {
        Self { configure_flags: None, jobs: Some(num_cpus::get()), default_extensions: None }
    }
}

pub fn load_or_default(config_path: &std::path::Path) -> anyhow::Result<Config> {
    if config_path.exists() {
        let text = fs::read_to_string(config_path)?;
        let cfg: Config = toml::from_str(&text)?;
        Ok(cfg)
    } else {
        Ok(Config::default())
    }
}

#[allow(dead_code)]
pub fn save(config_path: &std::path::Path, cfg: &Config) -> anyhow::Result<()> {
    let data = toml::to_string_pretty(cfg)?;
    if let Some(parent) = config_path.parent() { fs::create_dir_all(parent)?; }
    fs::write(config_path, data)?;
    Ok(())
}
