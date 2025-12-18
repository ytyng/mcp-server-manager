<script lang="ts">
  import { Toaster } from "svelte-sonner";
  import "../app.css";
  let { children } = $props();
</script>

<Toaster position="top-right" richColors />
{@render children()}
