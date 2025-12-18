export interface McpServer {
  name: string;
  enabled: boolean;
}

export interface Project {
  path: string;
  basename: string;
  disabledServers: string[];
}

export type TabType = "global" | "projects";
