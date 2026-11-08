<script lang="ts">
  import { EmptyState } from "@facile/muse";
  import { graph, selectNet, selection } from "../../lib/selection.svelte";

  const names = $derived(Object.keys(graph.nets).sort());
</script>

{#if names.length}
  <div class="flex flex-col gap-1">
    {#each names as name (name)}
      <button
        type="button"
        class="flex items-center justify-between gap-3 rounded-fc-lg px-3 py-2 text-left text-fc-sm hover:bg-fc-surface focus-visible:outline-2 focus-visible:outline-fc-ring {selection.net === name
          ? 'bg-fc-surface'
          : ''}"
        onclick={() => selectNet(name)}
      >
        <span class="truncate">{name}</span>
        <span class="shrink-0 text-fc-xs text-fc-fg-muted">{graph.nets[name].count} segments</span>
      </button>
    {/each}
  </div>
{:else}
  <EmptyState bare title="No nets detected" />
{/if}
