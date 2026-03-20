use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub timestamp_format: String,
    pub obsidian: ObsidianConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            timestamp_format: "%Y-%m-%d %H:%M".into(),
            obsidian: ObsidianConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ObsidianConfig {
    pub enabled: bool,
    pub vault_path: String,
    pub template_path: String,
    pub target_heading: String,
    pub entry_format: String,
}

impl Default for ObsidianConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            vault_path: String::new(),
            template_path: "daily/{{date:YYYY}}/{{date:YYYY-MM}}.md".into(),
            target_heading: "## Memos".into(),
            entry_format: "- {{timestamp}}: {{body}}".into(),
        }
    }
}

/// Returns the default config file path.
pub fn default_config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("Could not determine config directory")?
        .join("funpou");
    Ok(config_dir.join("config.toml"))
}

/// Load config from the given path, falling back to defaults if the file doesn't exist.
pub fn load_config(path: &Path) -> Result<Config> {
    if !path.exists() {
        return Ok(Config::default());
    }

    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;

    toml::from_str(&content)
        .with_context(|| format!("Failed to parse config file: {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_values() {
        let config = Config::default();
        assert_eq!(config.timestamp_format, "%Y-%m-%d %H:%M");
        assert!(!config.obsidian.enabled);
        assert!(config.obsidian.vault_path.is_empty());
        assert_eq!(
            config.obsidian.template_path,
            "daily/{{date:YYYY}}/{{date:YYYY-MM}}.md"
        );
        assert_eq!(config.obsidian.target_heading, "## Memos");
        assert_eq!(config.obsidian.entry_format, "- {{timestamp}}: {{body}}");
    }

    #[test]
    fn load_missing_config_returns_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nonexistent.toml");
        let config = load_config(&path).unwrap();
        assert_eq!(config.timestamp_format, "%Y-%m-%d %H:%M");
    }

    #[test]
    fn load_partial_config_merges_with_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");
        std::fs::write(&path, "timestamp_format = \"%m/%d %H:%M\"\n").unwrap();

        let config = load_config(&path).unwrap();
        assert_eq!(config.timestamp_format, "%m/%d %H:%M");
        // Obsidian defaults should still apply
        assert!(!config.obsidian.enabled);
    }

    #[test]
    fn load_full_obsidian_config() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");
        std::fs::write(
            &path,
            "timestamp_format = \"%Y-%m-%d %H:%M:%S\"\n\n\
             [obsidian]\n\
             enabled = true\n\
             vault_path = \"/tmp/vault\"\n\
             template_path = \"notes/{{date:YYYY-MM-DD}}.md\"\n\
             target_heading = \"## Quick Notes\"\n\
             entry_format = \"- {{body}} ({{timestamp}})\"\n",
        )
        .unwrap();

        let config = load_config(&path).unwrap();
        assert_eq!(config.timestamp_format, "%Y-%m-%d %H:%M:%S");
        assert!(config.obsidian.enabled);
        assert_eq!(config.obsidian.vault_path, "/tmp/vault");
        assert_eq!(
            config.obsidian.template_path,
            "notes/{{date:YYYY-MM-DD}}.md"
        );
        assert_eq!(config.obsidian.target_heading, "## Quick Notes");
    }

    #[test]
    fn config_serializes_to_toml() {
        let config = Config::default();
        let toml_str = toml::to_string_pretty(&config).unwrap();
        assert!(toml_str.contains("timestamp_format"));
        assert!(toml_str.contains("[obsidian]"));
    }
}
