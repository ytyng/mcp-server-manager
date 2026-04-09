<script lang="ts">
  import { onMount } from "svelte";
  import { toast } from "svelte-sonner";
  import { getMatches } from "@tauri-apps/plugin-cli";
  import TabContainer from "$lib/components/TabContainer.svelte";
  import GlobalSettings from "$lib/components/GlobalSettings.svelte";
  import DesktopSettings from "$lib/components/DesktopSettings.svelte";
  import ProjectSettings from "$lib/components/ProjectSettings.svelte";
  import {
    servers,
    desktopServers,
    projects,
    activeTab,
    projectFilter,
    isLoading,
  } from "$lib/stores";
  import { getMcpServers, getDesktopMcpServers, getProjects } from "$lib/api";

  onMount(async () => {
    // CLI 引数チェック
    try {
      const matches = await getMatches();
      if (matches.args.project?.value) {
        $activeTab = "projects";
        $projectFilter = matches.args.project.value as string;
      }
    } catch {
      // CLI plugin not available (web preview)
    }

    // データ読み込み（個別にエラーハンドリングし、一部失敗でも他のデータは表示する）
    const [serversResult, desktopServersResult, projectsResult] =
      await Promise.allSettled([
        getMcpServers(),
        getDesktopMcpServers(),
        getProjects(),
      ]);

    if (serversResult.status === "fulfilled") {
      $servers = serversResult.value;
    } else {
      toast.error(`Failed to load global config: ${serversResult.reason}`);
    }

    if (desktopServersResult.status === "fulfilled") {
      $desktopServers = desktopServersResult.value;
    } else {
      toast.error(`Failed to load desktop config: ${desktopServersResult.reason}`);
    }

    if (projectsResult.status === "fulfilled") {
      $projects = projectsResult.value;
    } else {
      toast.error(`Failed to load projects: ${projectsResult.reason}`);
    }

    $isLoading = false;
  });
</script>

{#if $isLoading}
  <div class="flex items-center justify-center h-screen bg-gray-100 dark:bg-gray-900 text-gray-500 dark:text-gray-400">
    Loading...
  </div>
{:else}
  <TabContainer>
    {#if $activeTab === "global"}
      <GlobalSettings />
    {:else if $activeTab === "claude-desktop"}
      <DesktopSettings />
    {:else}
      <ProjectSettings />
    {/if}
  </TabContainer>
{/if}
