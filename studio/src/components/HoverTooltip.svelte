<script lang="ts">
  import { selection } from "../lib/selection.svelte";

  const tip = $derived(selection.tip);
</script>

{#if tip}
  <div
    class="pointer-events-none fixed z-50 max-w-72 rounded-fc-lg bg-fc-surface px-4 py-3 text-fc-xs shadow-lg"
    style:left="{Math.min(tip.x + 14, window.innerWidth - 300)}px"
    style:top="{tip.y + 14}px"
    role="tooltip"
  >
    <div class="text-fc-sm font-medium">{tip.name}</div>
    {#if tip.type}<div class="text-fc-fg-muted">{tip.type}</div>{/if}
    {#each tip.rows as [key, value] (key)}
      <div class="mt-1 flex justify-between gap-4">
        <span class="text-fc-fg-muted">{key}</span>
        <span class={key === "Net" ? "text-fc-info" : ""}>{value}</span>
      </div>
    {/each}
  </div>
{/if}
