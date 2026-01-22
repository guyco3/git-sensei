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
            timeout_ms: 500,
            aggressive_minification: true,
            system_prompt: "You are a git commit assistant. Use Conventional Commits. Imperative mood. Max 72 chars. Return ONLY the message.".into(),
        }
    }
}

impl Config {
    pub fn load() -> Option<Self> {
        let path = Self::get_path();
        let content = fs::read_to_string(path).ok()?;
        toml::from_str(&content).ok()
    }

    pub fn get_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("gitsensei/config.toml")
    }
}