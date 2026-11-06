<script lang="ts">
  import { Icon, IconButton } from "@facile/muse";
  import { editLine, removePart, saveLater, saveBom } from "../lib/bom.svelte";
  import type { BomLine } from "../lib/types";

  let { line }: { line: BomLine } = $props();

  const price = (n: number | null | undefined) => (n == null ? "-" : n.toFixed(2));
  const safeUrl = (url?: string) => (/^https?:\/\//i.test(url || "") ? url : "");
  const field = "h-9 w-full rounded-full bg-fc-surface px-3 text-fc-sm text-fc-fg placeholder:text-fc-fg-muted focus-visible:outline-2 focus-visible:outline-fc-ring";

  function text(event: Event, name: string, rerender = false) {
    const control = event.currentTarget as HTMLInputElement;
    editLine(line, name, control.value.trim() || undefined);
    if (rerender) void saveBom();
    else saveLater();
  }

  function num(event: Event, name: string, rerender = false) {
    const control = event.currentTarget as HTMLInputElement;
    editLine(line, name, control.value === "" ? undefined : Number(control.value));
    if (rerender) void saveBom();
    else saveLater();
  }

  function pick(event: Event) {
    const checked = (event.currentTarget as HTMLInputElement).checked;
    editLine(line, "picked", checked ? undefined : false);
    saveLater();
  }

  const guess = $derived(line.estimated ? (line.kind === "print" ? `~${line.grams}` : `~${price(line.unit_price)}`) : "");
</script>

<div
  role="row"
  class="grid grid-cols-[2rem_4.5rem_minmax(10rem,1fr)_9rem_12rem_7rem_5rem_2.75rem] items-center gap-2 rounded-2xl bg-fc-component px-3 py-2 text-fc-sm {line.picked
    ? ''
    : 'opacity-50'}"
  data-id={line.id}
>
  <div role="cell"><input type="checkbox" class="size-4 accent-fc-accent" checked={line.picked} aria-label="Pick" onchange={pick} /></div>
  <div role="cell">
    <input class={field} type="number" min="0" step="1" value={line.quantity} aria-label="Quantity" onchange={(e) => num(e, "quantity")} />
  </div>
  <div role="cell" class="min-w-0">
    {#if line.kind === "manual"}
      <input class={field} value={line.description} aria-label="Part" onchange={(e) => text(e, "description")} />
    {:else}
      <div class="truncate">{line.description}</div>
      <div class="truncate text-fc-xs text-fc-fg-muted">{[line.mpn, line.designators.join(" ")].filter(Boolean).join(" · ")}</div>
    {/if}
  </div>
  <div role="cell">
    {#if line.kind !== "print"}
      <input class={field} value={line.supplier ?? ""} aria-label="Supplier" onchange={(e) => text(e, "supplier")} />
    {/if}
  </div>
  <div role="cell" class="flex items-center gap-1">
    {#if line.kind !== "print"}
      <input class={field} value={line.url ?? ""} placeholder="https://" aria-label="Link" onchange={(e) => text(e, "url")} />
      {#if safeUrl(line.url)}
        <a
          class="inline-flex size-9 shrink-0 items-center justify-center rounded-full hover:bg-fc-surface focus-visible:outline-2 focus-visible:outline-fc-ring"
          href={line.url}
          target="_blank"
          rel="noopener noreferrer"
          aria-label="Open link"
          title="Open link"
        >
          <Icon icon="solar:link-linear" size={16} />
        </a>
      {/if}
    {/if}
  </div>
  <div role="cell" class="flex items-center gap-1">
    {#if line.kind === "print"}
      <input
        class={field}
        type="number"
        min="0"
        step="1"
        value={line.estimated ? "" : (line.grams ?? "")}
        placeholder={guess}
        aria-label="Grams"
        onchange={(e) => num(e, "grams", true)}
      />
      <span class="text-fc-fg-muted">g</span>
    {:else}
      <input
        class={field}
        type="number"
        min="0"
        step="0.01"
        value={line.estimated ? "" : (line.unit_price ?? "")}
        placeholder={guess}
        aria-label="Unit price"
        onchange={(e) => num(e, "unit_price")}
      />
    {/if}
  </div>
  <div role="cell" class="text-right">{line.estimated && line.total != null ? "~" : ""}{price(line.total)}</div>
  <div role="cell">
    {#if line.kind === "manual"}
      <IconButton variant="ghost" class="size-9" aria-label="Remove part" title="Remove part" onclick={() => removePart(line.id)}>
        <Icon icon="solar:trash-bin-2-linear" />
      </IconButton>
    {/if}
  </div>
</div>
