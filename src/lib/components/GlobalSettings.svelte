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

<div class="space-y-1">
  <h2 class="text-lg font-semibold text-gray-800 dark:text-gray-200 mb-4">
    MCP Servers
  </h2>

  {#if $servers.length === 0}
    <p class="text-gray-500 dark:text-gray-400 text-sm">No servers found</p>
  {:else}
    {#each $servers as server (server.name)}
      <ServerCheckbox
        label={server.name}
        checked={server.enabled}
        onchange={(checked) => handleToggle(server.name, checked)}
      />
    {/each}
  {/if}
</div>
