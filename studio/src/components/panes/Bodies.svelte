<script lang="ts">
  import { EmptyState } from "@facile/muse";
  import { catalog } from "../../lib/catalog.svelte";
  import { project, selectItem } from "../../lib/project.svelte";
  import type { BodyInfo } from "../../lib/types";

  const bodies = $derived<BodyInfo[]>(
    catalog.bodies ?? (project.data?.bodies ?? []).map((body) => ({ name: body.name, kind: body.kind ?? "generated" })),
  );

  function detail(body: BodyInfo): string {
    const parts = [body.material, body.process, body.mass_g !== undefined ? `${body.mass_g} g` : undefined];
    if (body.cost !== undefined) parts.push(body.cost.toFixed(2));
    return parts.filter(Boolean).join(" · ");
  }
</script>

{#if bodies.length}
  <div class="flex flex-col gap-1">
    {#each bodies as body (body.name)}
      <button
        type="button"
        class="flex items-center justify-between gap-3 rounded-2xl px-4 py-2 text-left text-fc-sm hover:bg-fc-component focus-visible:outline-2 focus-visible:outline-fc-ring"
        onclick={() => selectItem("body", body.name)}
      >
        <span class="min-w-0">
          <span class="block truncate">{body.name}</span>
          <span class="block truncate text-fc-xs text-fc-fg-muted">{detail(body)}</span>
        </span>
        <span class="shrink-0 rounded-full bg-fc-component px-3 py-0.5 text-fc-xs text-fc-fg-muted">{body.kind}</span>
      </button>
    {/each}
  </div>
{:else}
  <EmptyState bare title="No bodies" description="Add a body in fusible.yml to see it here." />
{/if}
