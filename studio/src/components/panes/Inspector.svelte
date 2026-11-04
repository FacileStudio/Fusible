<script lang="ts">
  import { EmptyState } from "@facile/muse";
  import { lookup, selectNet, selection } from "../../lib/selection.svelte";

  const comp = $derived(lookup(selection.pinned) ?? selection.preview);
  const preview = $derived(!selection.pinned && comp !== null);
</script>

{#if comp}
  <div class="flex flex-col gap-2 text-fc-sm select-text">
    <div class="flex items-baseline justify-between gap-2 rounded-2xl bg-fc-component px-4 py-3">
      <span class="font-medium">{comp.name}{#if preview}<span class="ml-2 text-fc-xs text-fc-fg-muted">Preview</span>{/if}</span>
      <span class="text-fc-xs text-fc-fg-muted">{comp.ftype || "Component"}</span>
    </div>
    <dl class="flex flex-col gap-1 rounded-2xl bg-fc-component px-4 py-3">
      {#if comp.mpn}
        <div class="flex justify-between gap-4"><dt class="text-fc-fg-muted">Part</dt><dd>{comp.mpn}</dd></div>
      {/if}
      <div class="flex justify-between gap-4"><dt class="text-fc-fg-muted">Center</dt><dd>({comp.x.toFixed(2)}, {comp.y.toFixed(2)}) mm</dd></div>
      {#if comp.w && comp.h}
        <div class="flex justify-between gap-4"><dt class="text-fc-fg-muted">Footprint</dt><dd>{comp.w.toFixed(1)} x {comp.h.toFixed(1)} mm</dd></div>
      {/if}
      <div class="flex justify-between gap-4"><dt class="text-fc-fg-muted">Layer</dt><dd>{comp.layer || "top"}</dd></div>
    </dl>
    {#if comp.pins.length}
      <div class="flex flex-col gap-1 rounded-2xl bg-fc-component p-2">
        <div class="px-2 py-1 text-fc-xs text-fc-fg-muted">Connected pins and signals</div>
        {#each comp.pins as pin (pin.pin)}
          <button
            type="button"
            class="flex justify-between gap-4 rounded-full px-3 py-1.5 text-left hover:bg-fc-surface focus-visible:outline-2 focus-visible:outline-fc-ring"
            onclick={() => selectNet(pin.net)}
          >
            <span>{pin.pin}</span>
            <span class="text-fc-info">{pin.net}</span>
          </button>
        {/each}
      </div>
    {/if}
  </div>
{:else}
  <EmptyState bare title="Nothing selected" description="Select or hover a component or pad to inspect it." />
{/if}
