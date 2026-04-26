<script lang="ts">
  import { servers } from "$lib/stores";
  import { setServerEnabled, getMcpServers } from "$lib/api";
  import ServerListPanel from "./ServerListPanel.svelte";
</script>

<ServerListPanel
  title="MCP Servers (Claude Code)"
  servers={$servers}
  setEnabled={setServerEnabled}
  reload={getMcpServers}
  onupdate={(s) => ($servers = s)}
/>
