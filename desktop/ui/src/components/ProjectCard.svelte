<script lang="ts">
  import { Badge, Button, Icon, IconButton } from "@facile/muse";
  import { shortenPath } from "../lib/path";
  import { projects } from "../lib/projects.svelte";
  import { relativeTime } from "../lib/time";
  import type { Project } from "../lib/types";

  let { project }: { project: Project } = $props();

  const busy = $derived(projects.opening === project.path);
  const counts = $derived([
    { n: project.boards, one: "board", many: "boards" },
    { n: project.firmware, one: "firmware", many: "firmware" },
    { n: project.bodies, one: "body", many: "bodies" },
  ].filter((c) => c.n > 0));
</script>

<article class="flex items-center gap-4 rounded-3xl bg-fc-surface p-5">
  <div class="flex min-w-0 flex-1 flex-col gap-1.5">
    <h3 class="truncate text-fc-md font-semibold">{project.name}</h3>
    <p class="truncate text-fc-xs text-fc-fg-muted" title={project.path}>{shortenPath(project.path, 64)}</p>
    <div class="flex flex-wrap items-center gap-1.5">
      {#if project.missing}
        <Badge tone="danger">Folder missing</Badge>
      {/if}
      {#each counts as c (c.one)}
        <Badge>{c.n} {c.n === 1 ? c.one : c.many}</Badge>
      {:else}
        <Badge>No boards yet</Badge>
      {/each}
      <span class="text-fc-xs text-fc-fg-muted">{relativeTime(project.last_opened)}</span>
    </div>
  </div>
  <div class="flex shrink-0 items-center gap-1">
    <Button disabled={project.missing || projects.opening !== null} onclick={() => projects.open(project.path, project.name)}>
      {busy ? "Opening" : "Open"}
    </Button>
    <IconButton variant="ghost" aria-label="Reveal {project.name} in the file manager" onclick={() => projects.reveal(project.path)}>
      <Icon icon="solar:folder-open-linear" />
    </IconButton>
    <IconButton variant="danger" aria-label="Forget {project.name}" onclick={() => projects.forget(project.path)}>
      <Icon icon="solar:trash-bin-2-linear" />
    </IconButton>
  </div>
</article>
