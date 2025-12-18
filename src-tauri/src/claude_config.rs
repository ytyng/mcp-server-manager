use serde_json::Value;
use std::fs;
use std::path::PathBuf;

pub fn get_config_path() -> Result<PathBuf, String> {
    dirs::home_dir()
        .map(|p| p.join(".claude.json"))
        .ok_or_else(|| "Could not determine home directory".to_string())
}

pub fn read_config() -> Result<Value, String> {
    let path = get_config_path()?;
    let content =
        fs::read_to_string(&path).map_err(|e| format!("Failed to read config: {}", e))?;
    serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON: {}", e))
}

pub fn write_config(config: &Value) -> Result<(), String> {
    let path = get_config_path()?;
    let content =
        serde_json::to_string_pretty(config).map_err(|e| format!("Failed to serialize: {}", e))?;
    fs::write(&path, content).map_err(|e| format!("Failed to write: {}", e))
}
