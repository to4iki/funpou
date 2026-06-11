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
    pub vault_path: PathBuf,
    pub template_path: String,
    pub target_heading: String,
    /// strftime format with a `{body}` placeholder for the memo text.
    pub entry_format: String,
}

impl Default for ObsidianConfig {
    fn default() -> Self {
        Self {
            vault_path: PathBuf::new(),
            template_path: "daily/%Y-%m-%d.md".into(),
            target_heading: "## Memos".into(),
            entry_format: "- %Y-%m-%d %H:%M: {body}".into(),
        }
    }
}

impl ObsidianConfig {
    /// Returns true when vault_path is set to a non-empty value.
    pub fn is_enabled(&self) -> bool {
        !self.vault_path.as_os_str().is_empty()
    }
}

/// Expand a leading `~` or `~/` to the given home directory.
fn expand_tilde(path: &Path, home: Option<&Path>) -> PathBuf {
    let Some(home) = home else {
        return path.to_path_buf();
    };
    match path.to_str() {
        Some("~") => home.to_path_buf(),
        Some(s) => match s.strip_prefix("~/") {
            Some(rest) => home.join(rest),
            None => path.to_path_buf(),
        },
        None => path.to_path_buf(),
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
        assert!(!config.obsidian.is_enabled());
    }

    #[test]
    fn load_full_obsidian_config() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");
        std::fs::write(
            &path,
            "timestamp_format = \"%Y-%m-%d %H:%M:%S\"\n\n\
             [obsidian]\n\
             vault_path = \"/tmp/vault\"\n\
             template_path = \"notes/%Y-%m-%d.md\"\n\
             target_heading = \"## Quick Notes\"\n\
             entry_format = \"- {body} (%Y-%m-%d %H:%M)\"\n",
        )
        .unwrap();

        let config = load_config(&path).unwrap();
        assert_eq!(config.timestamp_format, "%Y-%m-%d %H:%M:%S");
        assert!(config.obsidian.is_enabled());
        assert_eq!(config.obsidian.vault_path, PathBuf::from("/tmp/vault"));
        assert_eq!(config.obsidian.template_path, "notes/%Y-%m-%d.md");
        assert_eq!(config.obsidian.target_heading, "## Quick Notes");
        assert_eq!(config.obsidian.entry_format, "- {body} (%Y-%m-%d %H:%M)");
    }

    #[test]
    fn expand_tilde_paths() {
        let home = Path::new("/Users/foo");
        assert_eq!(
            expand_tilde(Path::new("~/vault"), Some(home)),
            PathBuf::from("/Users/foo/vault")
        );
        assert_eq!(
            expand_tilde(Path::new("~"), Some(home)),
            PathBuf::from("/Users/foo")
        );
        assert_eq!(
            expand_tilde(Path::new("/abs/path"), Some(home)),
            PathBuf::from("/abs/path")
        );
        // `~user/path` is shell user-expansion, not supported here
        assert_eq!(
            expand_tilde(Path::new("~bar/path"), Some(home)),
            PathBuf::from("~bar/path")
        );
        assert_eq!(expand_tilde(Path::new("~/x"), None), PathBuf::from("~/x"));
    }

    #[test]
    fn config_path_resolution() {
        let home = Path::new("/Users/foo");
        let xdg = Path::new("/custom/xdg");
        assert_eq!(
            config_path_from(Some(home), Some(xdg)).unwrap(),
            PathBuf::from("/custom/xdg/funpou/config.toml")
        );
        assert_eq!(
            config_path_from(Some(home), None).unwrap(),
            PathBuf::from("/Users/foo/.config/funpou/config.toml")
        );
        assert!(config_path_from(None, None).is_none());
    }

    #[test]
    fn load_config_expands_tilde_in_vault_path() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.toml");
        std::fs::write(&path, "[obsidian]\nvault_path = \"~/valut/to4iki\"\n").unwrap();

        let home = Path::new("/Users/foo");
        let config = load_config_with_home(&path, Some(home)).unwrap();
        assert_eq!(
            config.obsidian.vault_path,
            PathBuf::from("/Users/foo/valut/to4iki")
        );
    }
}
