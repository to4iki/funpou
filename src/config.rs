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

/// Expand a leading `~` or `~/` to the given home directory.
fn expand_tilde(path: &str, home: Option<&Path>) -> String {
    match home {
        Some(home) if path == "~" => home.to_string_lossy().into_owned(),
        Some(home) => match path.strip_prefix("~/") {
            Some(rest) => home.join(rest).to_string_lossy().into_owned(),
            None => path.to_string(),
        },
        None => path.to_string(),
    }
}

/// Resolve the config file path from home and XDG_CONFIG_HOME, preferring XDG.
fn config_path_from(home: Option<&Path>, xdg_config_home: Option<&Path>) -> Option<PathBuf> {
    let base = xdg_config_home
        .map(PathBuf::from)
        .or_else(|| home.map(|h| h.join(".config")))?;
    Some(base.join("funpou").join("config.toml"))
}

/// Returns the default config file path.
/// Prefers `$XDG_CONFIG_HOME/funpou/config.toml`, falling back to `~/.config/funpou/config.toml`.
pub fn default_config_path() -> Result<PathBuf> {
    let home = dirs::home_dir();
    let xdg = std::env::var_os("XDG_CONFIG_HOME").map(PathBuf::from);
    config_path_from(home.as_deref(), xdg.as_deref())
        .context("Could not determine config directory")
}

/// Load config from the given path, falling back to defaults if the file doesn't exist.
pub fn load_config(path: &Path) -> Result<Config> {
    load_config_with_home(path, dirs::home_dir().as_deref())
}

/// Load config with an explicit home directory for tilde expansion (for testing).
fn load_config_with_home(path: &Path, home: Option<&Path>) -> Result<Config> {
    if !path.exists() {
        return Ok(Config::default());
    }

    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;

    let mut config: Config = toml::from_str(&content)
        .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

    config.obsidian.vault_path = expand_tilde(&config.obsidian.vault_path, home);

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn expand_tilde_with_home() {
        let home = Path::new("/Users/foo");
        assert_eq!(
            expand_tilde("~/valut/to4iki", Some(home)),
            "/Users/foo/valut/to4iki"
        );
        assert_eq!(expand_tilde("~", Some(home)), "/Users/foo");
    }

    #[test]
    fn expand_tilde_leaves_absolute_and_relative_paths() {
        let home = Path::new("/Users/foo");
        assert_eq!(expand_tilde("/abs/path", Some(home)), "/abs/path");
        assert_eq!(expand_tilde("rel/path", Some(home)), "rel/path");
        assert_eq!(expand_tilde("", Some(home)), "");
    }

    #[test]
    fn expand_tilde_only_handles_slash_prefix() {
        let home = Path::new("/Users/foo");
        // `~user/path` is shell user-expansion, not supported here
        assert_eq!(expand_tilde("~bar/path", Some(home)), "~bar/path");
    }

    #[test]
    fn expand_tilde_without_home_keeps_path() {
        assert_eq!(expand_tilde("~/x", None), "~/x");
        assert_eq!(expand_tilde("~", None), "~");
    }

    #[test]
    fn config_path_prefers_xdg_config_home() {
        let home = Path::new("/Users/foo");
        let xdg = Path::new("/custom/xdg");
        let path = config_path_from(Some(home), Some(xdg)).unwrap();
        assert_eq!(path, PathBuf::from("/custom/xdg/funpou/config.toml"));
    }

    #[test]
    fn config_path_falls_back_to_dot_config() {
        let home = Path::new("/Users/foo");
        let path = config_path_from(Some(home), None).unwrap();
        assert_eq!(path, PathBuf::from("/Users/foo/.config/funpou/config.toml"));
    }

    #[test]
    fn config_path_none_without_home_or_xdg() {
        assert!(config_path_from(None, None).is_none());
    }

    #[test]
    fn load_config_expands_tilde_in_vault_path() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");
        std::fs::write(
            &path,
            "[obsidian]\nenabled = true\nvault_path = \"~/valut/to4iki\"\n",
        )
        .unwrap();

        let home = Path::new("/Users/foo");
        let config = load_config_with_home(&path, Some(home)).unwrap();
        assert_eq!(config.obsidian.vault_path, "/Users/foo/valut/to4iki");
    }
}
