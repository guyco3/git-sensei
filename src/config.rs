use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub model: String,
    pub endpoint: String,
    pub timeout_ms: u64,
    pub aggressive_minification: bool,
    pub system_prompt: String, // Added this field
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model: "llama3.2:1b".into(),
            endpoint: "http://localhost:11434/api/generate".into(),
            timeout_ms: 3000,
            aggressive_minification: true,
            system_prompt: "You are a git commit assistant. Use Conventional Commits. Imperative mood. Max 72 chars. Return ONLY the message.".into(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let path = Self::get_path();

        // 1. Try to read the existing file
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(cfg) = toml::from_str(&content) {
                return cfg;
            }
        }

        // 2. If file missing or corrupt, generate defaults
        let default_cfg = Self::default();
        
        // 3. Try to save the defaults so the user can edit them later
        if let Err(e) = default_cfg.save() {
            eprintln!("Warning: Could not save default config: {}", e);
        }

        default_cfg
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::get_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?; // Create ~/.config/gitsensei/
        }
        let content = toml::to_string_pretty(self).unwrap();
        fs::write(path, content)
    }

    pub fn get_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("gitsensei/config.toml")
    }
}