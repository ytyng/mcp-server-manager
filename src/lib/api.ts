import { invoke } from "@tauri-apps/api/core";
import type { McpServer, Project } from "./types";

interface RawProject {
  path: string;
  basename: string;
  disabled_servers: string[];
}

export async function getMcpServers(): Promise<McpServer[]> {
  return await invoke<McpServer[]>("get_mcp_servers");
}

export async function setServerEnabled(
  name: string,
  enabled: boolean
): Promise<void> {
  await invoke("set_server_enabled", { name, enabled });
}

export async function getProjects(): Promise<Project[]> {
  const rawProjects = await invoke<RawProject[]>("get_projects");
  return rawProjects.map((p) => ({
    path: p.path,
    basename: p.basename,
    disabledServers: p.disabled_servers,
  }));
}

export async function setProjectDisabledServers(
  projectPath: string,
  disabledServers: string[]
): Promise<void> {
  await invoke("set_project_disabled_servers", {
    projectPath,
    disabledServers,
  });
}
