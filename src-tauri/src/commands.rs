use crate::claude_config::{read_config, write_config};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct McpServerInfo {
    pub name: String,
    pub enabled: bool,
}

#[derive(Serialize)]
pub struct ProjectInfo {
    pub path: String,
    pub basename: String,
    pub disabled_servers: Vec<String>,
}

/// グローバル MCP サーバー一覧取得
/// mcpServers と disabledMcpServers の両方から取得
#[tauri::command]
pub fn get_mcp_servers() -> Result<Vec<McpServerInfo>, String> {
    let config = read_config()?;
    let mut servers = Vec::new();

    // mcpServers (enabled)
    if let Some(mcp_servers) = config.get("mcpServers").and_then(|s| s.as_object()) {
        for name in mcp_servers.keys() {
            servers.push(McpServerInfo {
                name: name.clone(),
                enabled: true,
            });
        }
    }

    // disabledMcpServers (disabled)
    if let Some(disabled_servers) = config.get("disabledMcpServers").and_then(|s| s.as_object()) {
        for name in disabled_servers.keys() {
            servers.push(McpServerInfo {
                name: name.clone(),
                enabled: false,
            });
        }
    }

    servers.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(servers)
}

/// グローバルサーバーの有効/無効切替
/// enabled=true: disabledMcpServers -> mcpServers
/// enabled=false: mcpServers -> disabledMcpServers
#[tauri::command]
pub fn set_server_enabled(name: String, enabled: bool) -> Result<(), String> {
    let mut config = read_config()?;

    let (from_key, to_key) = if enabled {
        ("disabledMcpServers", "mcpServers")
    } else {
        ("mcpServers", "disabledMcpServers")
    };

    // 移動元からサーバー設定を取得して削除
    let server_config = {
        let from_servers = config
            .get_mut(from_key)
            .and_then(|s| s.as_object_mut())
            .ok_or_else(|| format!("No {} configured", from_key))?;

        from_servers
            .remove(&name)
            .ok_or_else(|| format!("Server '{}' not found", name))?
    };

    // 移動先に追加（存在しなければ作成）
    if config.get(to_key).is_none() {
        config
            .as_object_mut()
            .expect("config is always an object")
            .insert(to_key.to_string(), serde_json::json!({}));
    }

    let to_servers = config
        .get_mut(to_key)
        .and_then(|s| s.as_object_mut())
        .expect("just ensured to_key exists");
    to_servers.insert(name, server_config);

    write_config(&config)
}

/// プロジェクト一覧取得
#[tauri::command]
pub fn get_projects() -> Result<Vec<ProjectInfo>, String> {
    let config = read_config()?;
    let projects = config
        .get("projects")
        .and_then(|p| p.as_object())
        .ok_or("No projects configured in ~/.claude.json")?;

    let mut result: Vec<ProjectInfo> = projects
        .iter()
        .map(|(path, value)| {
            let disabled_servers = value
                .get("disabledMcpServers")
                .and_then(|d| d.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default();

            let basename = std::path::Path::new(path)
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| path.clone());

            ProjectInfo {
                path: path.clone(),
                basename,
                disabled_servers,
            }
        })
        .collect();

    result.sort_by(|a, b| a.basename.to_lowercase().cmp(&b.basename.to_lowercase()));
    Ok(result)
}

/// プロジェクトのサーバー無効化リスト更新
#[tauri::command]
pub fn set_project_disabled_servers(
    project_path: String,
    disabled_servers: Vec<String>,
) -> Result<(), String> {
    let mut config = read_config()?;

    let projects = config
        .get_mut("projects")
        .and_then(|p| p.as_object_mut())
        .ok_or("No projects configured in ~/.claude.json")?;

    let project = projects
        .get_mut(&project_path)
        .and_then(|p| p.as_object_mut())
        .ok_or_else(|| format!("Project '{}' not found", project_path))?;

    if disabled_servers.is_empty() {
        project.remove("disabledMcpServers");
    } else {
        project.insert(
            "disabledMcpServers".to_string(),
            Value::Array(disabled_servers.into_iter().map(Value::String).collect()),
        );
    }

    write_config(&config)
}
