<script lang="ts">
  import { onMount } from "svelte";
  import { toast } from "svelte-sonner";
  import { getMatches } from "@tauri-apps/plugin-cli";
  import TabContainer from "$lib/components/TabContainer.svelte";
  import GlobalSettings from "$lib/components/GlobalSettings.svelte";
  import ProjectSettings from "$lib/components/ProjectSettings.svelte";
  import {
    servers,
    projects,
    activeTab,
    projectFilter,
    isLoading,
  } from "$lib/stores";
  import { getMcpServers, getProjects } from "$lib/api";

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

    // データ読み込み
    try {
      $servers = await getMcpServers();
      $projects = await getProjects();
    } catch (e) {
      toast.error(`Failed to load config: ${e}`);
    } finally {
      $isLoading = false;
    }
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
    {:else}
      <ProjectSettings />
    {/if}
  </TabContainer>
{/if}
