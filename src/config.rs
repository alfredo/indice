use config::{Config as ConfigRS, ConfigError, File, FileFormat};
use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub enable_feature: bool,

    #[serde(default)]
    pub projects: HashMap<String, Vec<ProjectConfig>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enable_feature: true,
            projects: HashMap::new(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = Self::config_path()?;

        let s = ConfigRS::builder()
            .add_source(File::new(
                config_path.to_str().unwrap(),
                FileFormat::Yaml
            ))
            .build()?;

        s.try_deserialize()
    }

    pub fn config_path() -> Result<PathBuf, ConfigError> {
        // Look for .indicerc in the current directory first
        let current_dir = std::env::current_dir()
            .map_err(|e| ConfigError::Message(e.to_string()))?;
        let local_config = current_dir.join(".indicerc");

        if local_config.exists() {
            return Ok(local_config);
        }

        // Look for .indicerc in home directory
        if let Some(home_dir) = dirs::home_dir() {
            let home_config = home_dir.join(".indicerc");
            if home_config.exists() {
                return Ok(home_config);
            }
            // If file doesn't exist, return this path anyway for creation
            return Ok(home_config);
        }

        Err(ConfigError::Message("Could not determine home directory".to_string()))
    }

    pub fn create_default_config(path: &PathBuf) -> Result<(), ConfigError> {
        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| ConfigError::Message(e.to_string()))?;
        }

        let default_config = Config {
            enable_feature: true,
            projects: HashMap::new(),
        };

        // Create the YAML with comments explaining the format
        let yaml_str = format!(
            r#"# Configuration file for indice
# Enable or disable features
enable_feature: true

# Project definitions
projects:
  # example:
  #   - path: /absolute/path/to/project
"#
        );

        std::fs::write(path, yaml_str)
            .map_err(|e| ConfigError::Message(e.to_string()))?;

        Ok(())
    }
}
