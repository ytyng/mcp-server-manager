<script lang="ts">
  import { toast } from "svelte-sonner";
  import { servers } from "$lib/stores";
  import { setServerEnabled, getMcpServers } from "$lib/api";
  import ServerCheckbox from "./ServerCheckbox.svelte";

  async function handleToggle(name: string, enabled: boolean) {
    try {
      await setServerEnabled(name, enabled);
      $servers = await getMcpServers();
      toast.success(`${name} ${enabled ? "enabled" : "disabled"}`);
    } catch (e) {
      toast.error(`Failed to update ${name}: ${e}`);
    }
  }
</script>

<div class="max-h-full flex flex-col">
  <h2 class="text-lg font-semibold text-gray-800 dark:text-gray-200 bg-gray-100 dark:bg-gray-700 px-4 py-3">
    MCP Servers (User)
  </h2>

  {#if $servers.length === 0}
    <p class="text-gray-500 dark:text-gray-400 text-sm p-4">No servers found</p>
  {:else}
    <div class="p-4 grow overflow-y-auto">
      {#each $servers as server (server.name)}
        <ServerCheckbox
          label={server.name}
          checked={server.enabled}
          onchange={(checked) => handleToggle(server.name, checked)}
        />
      {/each}
    </div>
  {/if}
</div>
