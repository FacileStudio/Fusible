<script lang="ts">
  import { EmptyState } from "@facile/muse";
  import { graph, selectComponent, selection } from "../../lib/selection.svelte";

  const names = $derived(Object.keys(graph.components).sort());
</script>

{#if names.length}
  <div class="flex flex-col gap-1">
    {#each names as name (name)}
      {@const comp = graph.components[name]}
      <button
        type="button"
        class="flex items-center justify-between gap-3 rounded-fc-lg px-3 py-2 text-left text-fc-sm hover:bg-fc-surface focus-visible:outline-2 focus-visible:outline-fc-ring {selection.pinned === name
          ? 'bg-fc-surface'
          : ''}"
        onclick={() => selectComponent(name)}
      >
        <span class="min-w-0">
          <span class="block truncate">{name}</span>
          <span class="block truncate text-fc-xs text-fc-fg-muted">{comp.mpn || comp.ftype || ""}</span>
        </span>
        <span class="shrink-0 text-fc-xs text-fc-fg-muted">({comp.x.toFixed(1)}, {comp.y.toFixed(1)})</span>
      </button>
    {/each}
  </div>
{:else}
  <EmptyState bare title="No components parsed" />
{/if}
