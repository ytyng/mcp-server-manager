<script lang="ts">
  import { projects, servers, projectFilter } from "$lib/stores";
  import ProjectCard from "./ProjectCard.svelte";

  let filteredProjects = $derived(
    $projects.filter(
      (p) =>
        $projectFilter === "" ||
        p.path.toLowerCase().includes($projectFilter.toLowerCase()) ||
        p.basename.toLowerCase().includes($projectFilter.toLowerCase())
    )
  );

  let serverNames = $derived($servers.map((s) => s.name));
</script>

<div class="flex flex-col h-full">
  <div class="px-4 pt-4 pb-3 bg-gray-100 dark:bg-gray-700">
    <input
      type="text"
      bind:value={$projectFilter}
      placeholder="Filter projects..."
      class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600
             rounded-lg bg-white dark:bg-gray-700
             text-gray-900 dark:text-white
             placeholder-gray-400 dark:placeholder-gray-500
             focus:ring-2 focus:ring-blue-500 focus:border-transparent
             outline-none transition-shadow"
    />

    <div class="text-sm text-gray-500 dark:text-gray-400">
      {filteredProjects.length} / {$projects.length} projects
    </div>
  </div>

  <div class="overflow-y-auto grow p-4 flex flex-col gap-4">
    {#each filteredProjects as project (project.path)}
      <ProjectCard {project} {serverNames} />
    {/each}
  </div>
</div>
