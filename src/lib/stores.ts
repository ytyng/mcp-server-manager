import { writable } from "svelte/store";
import type { McpServer, Project, TabType } from "./types";

export const servers = writable<McpServer[]>([]);
export const projects = writable<Project[]>([]);
export const activeTab = writable<TabType>("global");
export const projectFilter = writable<string>("");
export const isLoading = writable<boolean>(true);
