use std::path::Path;

use anyhow::Result;

use crate::config::Config;

pub fn execute(config: &Config, config_path: &Path, path_only: bool) -> Result<()> {
    if path_only {
        println!("{}", config_path.display());
        return Ok(());
    }

    let toml_str = toml::to_string_pretty(config)?;
    println!("{toml_str}");

    Ok(())
}
