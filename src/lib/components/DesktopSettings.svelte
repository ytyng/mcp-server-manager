<script lang="ts">
  import { desktopServers } from "$lib/stores";
  import { setDesktopServerEnabled, getDesktopMcpServers } from "$lib/api";
  import ServerListPanel from "./ServerListPanel.svelte";
</script>

<ServerListPanel
  title="MCP Servers (Claude Desktop)"
  servers={$desktopServers}
  setEnabled={setDesktopServerEnabled}
  reload={getDesktopMcpServers}
  onupdate={(s) => ($desktopServers = s)}
/>
