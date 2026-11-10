<script lang="ts">
  import { onMount } from "svelte";
  import { Button, EmptyState, Icon } from "@facile/muse";
  import { api } from "../../lib/api";
  import { catalog, loadRules } from "../../lib/catalog.svelte";
  import { categoryLabel, fixLabel, groupByCategory, nearestPcbComponent, setPropFixes, tone } from "../../lib/checks";
  import { getCircuit, setView, showStatus, studio } from "../../lib/studio.svelte";
  import { Tscircuit } from "../../tscircuit";
  import type { Diagnostic, SetProp } from "../../lib/types";

  const groups = $derived(groupByCategory(studio.diagnostics, catalog.rules));
  let applying = $state(false);

  const TONE_TEXT = { danger: "text-fc-danger", warning: "text-fc-warning", info: "text-fc-info" } as const;
  const TONE_ICON = {
    danger: "solar:danger-triangle-bold-duotone",
    warning: "solar:danger-circle-bold-duotone",
    info: "solar:info-circle-bold-duotone",
  } as const;

  onMount(() => {
    if (!catalog.rules.length) void loadRules();
  });

  function zoom(diag: Diagnostic): void {
    if (!diag.location) return;
    const target = nearestPcbComponent(getCircuit(), diag.location.x, diag.location.y);
    setView("pcb");
    if (target) Tscircuit.focusPcbComponent(target);
  }

  async function applyFix(fix: SetProp): Promise<void> {
    applying = true;
    try {
      await api.applyFix(studio.board, fix);
      showStatus(`${fixLabel(fix)}: applied, rebuilding`);
    } catch (err) {
      showStatus(`Fix not applied: ${err instanceof Error ? err.message : String(err)}`);
    } finally {
      applying = false;
    }
  }
</script>

{#snippet standard(diag: Diagnostic)}
  {#if diag.standard}
    <div class="flex flex-wrap items-center gap-2 text-fc-xs">
      {#if diag.source_url}
        <a
          class="inline-flex h-7 items-center rounded-fc-pill bg-fc-field px-3 hover:text-fc-info focus-visible:outline-2 focus-visible:outline-fc-ring"
          href={diag.source_url}
          target="_blank"
          rel="noopener noreferrer"
        >
          {diag.standard}
        </a>
      {:else}
        <span class="inline-flex h-7 items-center rounded-fc-pill bg-fc-field px-3">{diag.standard}</span>
      {/if}
      <span class="text-fc-fg-muted">Design pre-check, not certification.</span>
    </div>
  {/if}
{/snippet}

{#snippet card(diag: Diagnostic)}
  {@const level = tone(diag.severity)}
  <div class="flex flex-col gap-2 rounded-fc-lg bg-fc-surface px-4 py-3 text-fc-sm select-text">
    <div class="flex items-center justify-between gap-2">
      <div class="flex items-center gap-2 font-medium {TONE_TEXT[level]}">
        <Icon icon={TONE_ICON[level]} size={16} />
        <span>{diag.rule || "violation"}</span>
      </div>
      {#if diag.location}
        <Button size="sm" variant="ghost" icon="solar:magnifer-zoom-in-linear" onclick={() => zoom(diag)}>
          Show
        </Button>
      {/if}
    </div>
    <p>{diag.message || ""}</p>
    {#if diag.help}<p class="text-fc-fg-muted">{diag.help}</p>{/if}
    {#if diag.fix}<p class="text-fc-success">Fix: {diag.fix}</p>{/if}
    {#each setPropFixes(diag) as fix (fix.selector + fix.prop)}
      <Button size="sm" icon="solar:check-circle-linear" disabled={applying} onclick={() => applyFix(fix)}>
        Apply fix: {fixLabel(fix)}
      </Button>
    {/each}
    {@render standard(diag)}
  </div>
{/snippet}

{#if studio.diagnostics.length || studio.waived.length}
  <div class="flex flex-col gap-4">
    {#each groups as group (group.category)}
      <section class="flex flex-col gap-2" aria-label={categoryLabel(group.category)}>
        <h3 class="px-2 text-fc-xs font-medium text-fc-fg-muted">
          {categoryLabel(group.category)} · {group.items.length}
        </h3>
        {#each group.items as item (item.index)}
          {@render card(item.diag)}
        {/each}
      </section>
    {/each}
    {#if studio.waived.length}
      <section class="flex flex-col gap-2" aria-label="Waived">
        <h3 class="px-2 text-fc-xs font-medium text-fc-fg-muted">Waived · {studio.waived.length}</h3>
        {#each studio.waived as waived, index (index)}
          <div class="flex flex-col gap-1 rounded-fc-lg bg-fc-surface px-4 py-3 text-fc-sm text-fc-fg-muted select-text">
            <span class="font-medium">{waived.diagnostic.rule}</span>
            <p>{waived.diagnostic.message}</p>
            <p>Waived: {waived.reason}{waived.until ? ` (until ${waived.until})` : ""}</p>
          </div>
        {/each}
      </section>
    {/if}
  </div>
{:else}
  <EmptyState bare title="No electrical violations detected" />
{/if}
