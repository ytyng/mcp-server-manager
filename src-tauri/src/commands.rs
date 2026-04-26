use crate::claude_config::{
    read_config, read_desktop_config, read_desktop_manager_config, write_config,
    write_desktop_config, write_desktop_manager_config,
};
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
    let mut seen = std::collections::HashSet::new();

    // mcpServers (enabled) を優先（不整合時に重複を防ぐ）
    if let Some(mcp_servers) = config.get("mcpServers").and_then(|s| s.as_object()) {
        for name in mcp_servers.keys() {
            seen.insert(name.clone());
            servers.push(McpServerInfo {
                name: name.clone(),
                enabled: true,
            });
        }
    }

    // disabledMcpServers (disabled) - 既出はスキップ
    if let Some(disabled_servers) = config.get("disabledMcpServers").and_then(|s| s.as_object()) {
        for name in disabled_servers.keys() {
            if seen.insert(name.clone()) {
                servers.push(McpServerInfo {
                    name: name.clone(),
                    enabled: false,
                });
            }
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
            .ok_or("config root is not an object")?
            .insert(to_key.to_string(), serde_json::json!({}));
    }

    let to_servers = config
        .get_mut(to_key)
        .and_then(|s| s.as_object_mut())
        .ok_or_else(|| format!("{} is not an object", to_key))?;
    to_servers.insert(name, server_config);

    write_config(&config)
}

/// Claude Desktop MCP サーバー一覧取得
/// enabled: claude_desktop_config.json の mcpServers
/// disabled: mcp-server-manager-config.json の disabledMcpServers
#[tauri::command]
pub fn get_desktop_mcp_servers() -> Result<Vec<McpServerInfo>, String> {
    let desktop_config = read_desktop_config()?;
    let manager_config = read_desktop_manager_config()?;
    let mut servers = Vec::new();
    let mut seen = std::collections::HashSet::new();

    // enabled サーバーを優先（不整合時に重複を防ぐ）
    if let Some(mcp_servers) = desktop_config.get("mcpServers").and_then(|s| s.as_object()) {
        for name in mcp_servers.keys() {
            seen.insert(name.clone());
            servers.push(McpServerInfo {
                name: name.clone(),
                enabled: true,
            });
        }
    }

    if let Some(disabled) = manager_config.get("disabledMcpServers").and_then(|s| s.as_object()) {
        for name in disabled.keys() {
            if seen.insert(name.clone()) {
                servers.push(McpServerInfo {
                    name: name.clone(),
                    enabled: false,
                });
            }
        }
    }

    servers.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(servers)
}

/// Claude Desktop サーバーの有効/無効切替
/// 無効化: claude_desktop_config.json から削除 → mcp-server-manager-config.json に退避
/// 有効化: mcp-server-manager-config.json から削除 → claude_desktop_config.json に復元
#[tauri::command]
pub fn set_desktop_server_enabled(name: String, enabled: bool) -> Result<(), String> {
    let mut desktop_config = read_desktop_config()?;
    let mut manager_config = read_desktop_manager_config()?;

    if enabled {
        // manager の disabledMcpServers から取り出して desktop の mcpServers に戻す
        let server_config = {
            let disabled = manager_config
                .get_mut("disabledMcpServers")
                .and_then(|s| s.as_object_mut())
                .ok_or("No disabledMcpServers in manager config")?;
            disabled
                .remove(&name)
                .ok_or_else(|| format!("Server '{}' not found in disabled servers", name))?
        };

        if desktop_config.get("mcpServers").is_none() {
            desktop_config
                .as_object_mut()
                .ok_or("desktop config root is not an object")?
                .insert("mcpServers".to_string(), serde_json::json!({}));
        }
        desktop_config
            .get_mut("mcpServers")
            .and_then(|s| s.as_object_mut())
            .ok_or("mcpServers is not an object")?
            .insert(name, server_config);

        write_desktop_config(&desktop_config)?;
        write_desktop_manager_config(&manager_config)
    } else {
        // desktop の mcpServers から取り出して manager の disabledMcpServers に退避
        let server_config = {
            let mcp_servers = desktop_config
                .get_mut("mcpServers")
                .and_then(|s| s.as_object_mut())
                .ok_or("No mcpServers in desktop config")?;
            mcp_servers
                .remove(&name)
                .ok_or_else(|| format!("Server '{}' not found in mcpServers", name))?
        };

        if manager_config.get("disabledMcpServers").is_none() {
            manager_config
                .as_object_mut()
                .ok_or("manager config root is not an object")?
                .insert("disabledMcpServers".to_string(), serde_json::json!({}));
        }
        manager_config
            .get_mut("disabledMcpServers")
            .and_then(|s| s.as_object_mut())
            .ok_or("disabledMcpServers is not an object")?
            .insert(name, server_config);

        write_desktop_config(&desktop_config)?;
        write_desktop_manager_config(&manager_config)
    }
}

/// プロジェクト一覧取得
#[tauri::command]
pub fn get_projects() -> Result<Vec<ProjectInfo>, String> {
    let config = read_config()?;
    // projects キーが無い場合は空リストを返す（新規環境では正常な状態）
    let Some(projects) = config.get("projects").and_then(|p| p.as_object()) else {
        return Ok(Vec::new());
    };

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
