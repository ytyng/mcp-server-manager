<script lang="ts">
  import { activeTab } from "$lib/stores";
  import type { Snippet } from "svelte";

  let { children }: { children: Snippet } = $props();

  const tabs = [
    { id: "global", label: "Global" },
    { id: "projects", label: "Projects" },
  ] as const;
</script>

<div class="bg-white dark:bg-gray-800 rounded-lg shadow">
  <div class="border-b border-gray-200 dark:border-gray-700">
    <nav class="flex" aria-label="Tabs">
      {#each tabs as tab}
        <button
          onclick={() => ($activeTab = tab.id)}
          class="flex-1 py-3 px-4 text-sm font-medium border-b-2 transition-colors
            {$activeTab === tab.id
            ? 'border-blue-500 text-blue-600 dark:text-blue-400 bg-blue-50 dark:bg-blue-900/20'
            : 'border-transparent text-gray-500 hover:text-gray-700 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-700/50'}"
        >
          {tab.label}
        </button>
      {/each}
    </nav>
  </div>

  <div class="p-4">
    {@render children()}
  </div>
</div>
