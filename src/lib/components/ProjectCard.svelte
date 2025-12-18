<script lang="ts">
  import { toast } from "svelte-sonner";
  import type { Project } from "$lib/types";
  import { setProjectDisabledServers, getProjects } from "$lib/api";
  import { projects } from "$lib/stores";
  import ServerCheckbox from "./ServerCheckbox.svelte";

  let {
    project,
    serverNames,
  }: {
    project: Project;
    serverNames: string[];
  } = $props();

  let expanded = $state(false);

  async function handleToggle(serverName: string, enabled: boolean) {
    try {
      let newDisabled: string[];
      if (enabled) {
        newDisabled = project.disabledServers.filter((s) => s !== serverName);
      } else {
        newDisabled = [...project.disabledServers, serverName];
      }
      await setProjectDisabledServers(project.path, newDisabled);
      $projects = await getProjects();
      toast.success(`${serverName} ${enabled ? "enabled" : "disabled"} for ${project.basename}`);
    } catch (e) {
      toast.error(`Failed to update: ${e}`);
    }
  }
</script>

<div class="border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden shrink-0">
  <button
    onclick={() => (expanded = !expanded)}
    class="w-full p-3 text-left hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors flex items-center justify-between"
  >
    <div class="min-w-0 flex-1">
      <h3 class="text-base font-semibold text-gray-900 dark:text-white truncate">
        {project.basename}
      </h3>
      <p class="text-xs text-gray-500 dark:text-gray-400 truncate">
        {project.path}
      </p>
    </div>
    <div class="ml-2 flex items-center gap-2">
      {#if project.disabledServers.length > 0}
        <span class="text-xs text-orange-500 bg-orange-100 dark:bg-orange-900/30 px-2 py-0.5 rounded">
          {project.disabledServers.length} disabled
        </span>
      {/if}
      <svg
        class="w-5 h-5 text-gray-400 transition-transform {expanded ? 'rotate-180' : ''}"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
      </svg>
    </div>
  </button>

  {#if expanded}
    <div class="border-t border-gray-200 dark:border-gray-700 p-3 bg-gray-50 dark:bg-gray-800/50">
      <div class="space-y-0.5">
        {#each serverNames as serverName (serverName)}
          <ServerCheckbox
            label={serverName}
            checked={!project.disabledServers.includes(serverName)}
            onchange={(checked) => handleToggle(serverName, checked)}
          />
        {/each}
      </div>
    </div>
  {/if}
</div>
