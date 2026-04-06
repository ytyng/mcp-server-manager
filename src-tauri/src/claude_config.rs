use serde_json::Value;
use std::fs;
use std::path::PathBuf;

fn home_dir() -> Result<PathBuf, String> {
    dirs::home_dir().ok_or_else(|| "Could not determine home directory".to_string())
}

/// ファイルが存在しない場合は空オブジェクトを返す
fn read_json(path: &PathBuf, default_if_missing: bool) -> Result<Value, String> {
    if default_if_missing && !path.exists() {
        return Ok(serde_json::json!({}));
    }
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse {}: {}", path.display(), e))
}

fn write_json(path: &PathBuf, config: &Value) -> Result<(), String> {
    let content = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize: {}", e))?;
    fs::write(path, content).map_err(|e| format!("Failed to write {}: {}", path.display(), e))
}

// --- Claude Code (~/.claude.json) ---

pub fn get_config_path() -> Result<PathBuf, String> {
    home_dir().map(|p| p.join(".claude.json"))
}

pub fn read_config() -> Result<Value, String> {
    read_json(&get_config_path()?, false)
}

pub fn write_config(config: &Value) -> Result<(), String> {
    write_json(&get_config_path()?, config)
}

// --- Claude Desktop (~/Library/Application Support/Claude/claude_desktop_config.json) ---

pub fn get_desktop_config_path() -> Result<PathBuf, String> {
    home_dir().map(|p| p.join("Library/Application Support/Claude/claude_desktop_config.json"))
}

/// 存在しない場合は空オブジェクトを返す（Claude Desktop 未インストールの場合）
pub fn read_desktop_config() -> Result<Value, String> {
    read_json(&get_desktop_config_path()?, true)
}

pub fn write_desktop_config(config: &Value) -> Result<(), String> {
    write_json(&get_desktop_config_path()?, config)
}

// --- 無効化サーバー退避先（Claude Desktop が未知キーを消すため別ファイルに保持）---

pub fn get_desktop_manager_config_path() -> Result<PathBuf, String> {
    home_dir().map(|p| {
        p.join("Library/Application Support/Claude/mcp-server-manager-config.json")
    })
}

pub fn read_desktop_manager_config() -> Result<Value, String> {
    read_json(&get_desktop_manager_config_path()?, true)
}

pub fn write_desktop_manager_config(config: &Value) -> Result<(), String> {
    write_json(&get_desktop_manager_config_path()?, config)
}
