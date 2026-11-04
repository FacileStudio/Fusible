<script lang="ts">
  import { Alert, Button, Icon, Input } from "@facile/muse";
  import { projects } from "../lib/projects.svelte";
  import EmptyProjects from "./EmptyProjects.svelte";
  import NewProject from "./NewProject.svelte";
  import ProjectCard from "./ProjectCard.svelte";

  let creating = $state(false);

  async function openFolder(): Promise<void> {
    if (await projects.openFolder()) creating = true;
  }

  function startNew(): void {
    projects.prefill = null;
    creating = true;
  }
</script>

<section class="mx-auto flex max-w-3xl flex-col gap-6">
  <header class="flex items-center justify-between gap-4">
    <h1 class="text-fc-2xl font-semibold">Projects</h1>
    <div class="flex items-center gap-2">
      <Button variant="ghost" icon="solar:folder-open-linear" onclick={openFolder}>Open folder</Button>
      <Button icon="solar:add-circle-linear" onclick={startNew}>New project</Button>
    </div>
  </header>

  {#if projects.failure}
    <Alert tone="danger" title={projects.failure.title}>
      <pre class="mt-2 max-h-48 overflow-auto whitespace-pre-wrap text-fc-xs">{projects.failure.log}</pre>
      <Button variant="ghost-danger" size="sm" class="mt-2" onclick={() => (projects.failure = null)}>Dismiss</Button>
    </Alert>
  {/if}

  {#if projects.loadError}
    <Alert tone="danger" title="Could not load projects">{projects.loadError}</Alert>
  {:else if projects.all.length === 0}
    <EmptyProjects onNew={startNew} onOpen={openFolder} />
  {:else}
    <label class="relative block">
      <span class="sr-only">Filter projects</span>
      <Icon icon="solar:magnifer-linear" class="pointer-events-none absolute left-4 top-1/2 -translate-y-1/2 text-fc-fg-muted" />
      <Input bind:value={projects.query} placeholder="Filter by name or path" class="rounded-full border-0 bg-fc-surface pl-11" />
    </label>
    <div class="flex flex-col gap-3">
      {#each projects.visible as project (project.path)}
        <ProjectCard {project} />
      {:else}
        <p class="py-8 text-center text-fc-sm text-fc-fg-muted">No project matches the filter.</p>
      {/each}
    </div>
  {/if}
</section>

<NewProject bind:open={creating} />
