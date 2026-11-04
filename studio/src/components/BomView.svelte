<script lang="ts">
  import { onMount } from "svelte";
  import { Button, EmptyState, Select } from "@facile/muse";
  import BomRow from "./BomRow.svelte";
  import { addPart, bom, loadBom, saveBom, saveLater } from "../lib/bom.svelte";

  const GROUPS = [
    ["component", "Electronics"],
    ["print", "3D prints"],
    ["manual", "Other parts"],
  ] as const;
  const CURRENCIES = ["EUR", "USD", "GBP", "CHF", "CAD", "AUD", "JPY", "CNY", "SEK", "NOK", "DKK", "PLN"];
  const price = (n: number | null | undefined) => (n == null ? "-" : n.toFixed(2));
  const field = "h-9 rounded-full bg-fc-component px-3 text-fc-sm text-fc-fg placeholder:text-fc-fg-muted focus-visible:outline-2 focus-visible:outline-fc-ring";

  let draft = $state("");
  const report = $derived(bom.data?.report);
  const currencies = $derived(report && !CURRENCIES.includes(report.currency) ? [report.currency, ...CURRENCIES] : CURRENCIES);

  function groupSum(kind: string): string {
    const lines = report?.lines.filter((l) => l.kind === kind && l.picked && l.total != null) ?? [];
    return `${price(lines.reduce((n, l) => n + (l.total ?? 0), 0))} ${report?.currency ?? ""}`;
  }

  function add() {
    void addPart(draft);
    draft = "";
  }

  function setCurrency(event: Event) {
    if (!bom.data) return;
    bom.data.file.currency = (event.currentTarget as HTMLSelectElement).value.trim() || undefined;
    void saveBom();
  }

  function setKg(event: Event) {
    if (!bom.data) return;
    bom.data.file.print.price_per_kg = Number((event.currentTarget as HTMLInputElement).value) || 0;
    saveLater();
  }

  onMount(() => {
    void loadBom();
  });
</script>

<section class="absolute inset-0 z-20 overflow-auto bg-fc-component p-6" aria-label="Parts and costs">
  {#if !bom.data}
    <EmptyState bare title={bom.error || "Loading parts..."} />
  {:else if report}
    <div class="mx-auto flex max-w-6xl flex-col gap-4 pb-20">
      <div>
        <h2 class="text-fc-xl font-medium">Parts and costs</h2>
        <p class="text-fc-sm text-fc-fg-muted">Saved to bom.yml</p>
      </div>
      {#if !bom.data.editable}
        <div class="rounded-2xl bg-fc-warning/10 px-4 py-3 text-fc-sm text-fc-warning">No fusible.yml: edits cannot be saved.</div>
      {/if}
      {#if bom.data.pending}
        <div class="rounded-2xl bg-fc-warning/10 px-4 py-3 text-fc-sm text-fc-warning">{bom.data.pending} board(s) still building.</div>
      {/if}
      {#if bom.error}<div class="rounded-2xl bg-fc-danger/10 px-4 py-3 text-fc-sm text-fc-danger" role="alert">{bom.error}</div>{/if}
      <div class="grid grid-cols-2 gap-2 md:grid-cols-4">
        {#each [["Electronics", report.totals.electronics], ["3D prints", report.totals.printed], ["Other parts", report.totals.manual], ["Total", report.totals.total]] as [label, value] (label)}
          <div class="rounded-3xl px-5 py-4 {label === 'Total' ? 'bg-fc-accent text-fc-accent-fg' : 'bg-fc-surface'}">
            <div class="text-fc-xs opacity-70">{label}</div>
            <div class="text-fc-xl font-medium">{price(value as number)} <span class="text-fc-sm opacity-70">{report.currency}</span></div>
          </div>
        {/each}
      </div>
      {#if report.totals.unpriced}
        <div class="text-fc-sm text-fc-warning">{report.totals.unpriced} picked line(s) have no price yet.</div>
      {/if}
      {#if report.totals.estimated}
        <div class="text-fc-sm text-fc-fg-muted">
          {report.totals.estimated} picked line(s) use an estimate (~). Type a price to replace it.
          {#if report.estimate_currency !== report.currency}Estimates are in {report.estimate_currency}: no rate for {report.currency}.{/if}
        </div>
      {/if}
      <div class="flex flex-wrap items-center gap-3 text-fc-sm">
        <label class="flex items-center gap-2">
          Currency
          <Select value={report.currency} class="h-9 w-auto rounded-fc-pill border-0 bg-fc-surface text-fc-sm" onchange={setCurrency}>
            {#each currencies as code (code)}<option>{code}</option>{/each}
          </Select>
        </label>
        <label class="flex items-center gap-2">
          Filament per kg
          <input class="{field} w-24" type="number" min="0" step="0.5" value={report.price_per_kg} onchange={setKg} />
        </label>
        <form class="flex items-center gap-2" onsubmit={(e) => (e.preventDefault(), add())}>
          <input class="{field} w-56" placeholder="Add a part, such as M3 screw" aria-label="Add a part" bind:value={draft} />
          <Button size="sm" type="submit" icon="solar:add-circle-linear">Add</Button>
        </form>
      </div>
      <div class="overflow-x-auto" role="table" aria-label="Parts">
        <div class="flex min-w-248 flex-col gap-1">
          {#each GROUPS as [kind, label] (kind)}
            {@const lines = report.lines.filter((l) => l.kind === kind)}
            {#if lines.length}
              <div class="flex items-center justify-between px-4 pt-3 text-fc-xs text-fc-fg-muted">
                <span>{label} &middot; {lines.length}</span><b>{groupSum(kind)}</b>
              </div>
              {#each lines as line (line.id)}<BomRow {line} />{/each}
            {/if}
          {/each}
          {#if !report.lines.length}
            <EmptyState bare title="No parts yet" description="Add one above." />
          {/if}
        </div>
      </div>
    </div>
  {/if}
</section>
