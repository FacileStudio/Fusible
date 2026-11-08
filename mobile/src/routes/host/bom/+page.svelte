<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { EmptyState } from "@facile/muse";
  import { get } from "$lib/client";
  import { Current } from "$lib/current.svelte";
  import { useT } from "$lib/i18n";

  type Line = {
    id: string;
    description: string;
    value?: string | null;
    designators: string[];
    quantity: number;
    total?: number | null;
  };
  type Bom = { report: { currency: string; lines: Line[]; totals: { total: number } } };

  const t = useT();
  const cur = new Current();
  const id = $derived(page.url.searchParams.get("id") ?? "");
  let bom = $state<Bom | null>(null);

  onMount(async () => {
    await cur.load(id);
    if (cur.host) bom = await get<Bom>(cur.host, "/api/bom");
  });

  const money = (a: number | null | undefined, c: string) =>
    a == null ? "-" : new Intl.NumberFormat(undefined, { style: "currency", currency: c }).format(a);
</script>

<main class="mx-auto flex max-w-xl flex-col gap-4 px-4 pt-8 pb-nav">
  <h1 class="text-fc-2xl font-semibold">{t("navBom")}</h1>
  {#if bom}
    {#each bom.report.lines as line (line.id)}
      <div class="flex items-center justify-between gap-3 rounded-fc-lg bg-fc-surface p-5">
        <div class="min-w-0">
          <span class="block truncate text-fc-md font-medium">{line.value ?? line.description}</span>
          <span class="block truncate text-fc-xs text-fc-fg-muted">
            {line.designators.join(", ")} · {t("qty")} {line.quantity}
          </span>
        </div>
        <span class="shrink-0 text-fc-md font-semibold">{money(line.total, bom.report.currency)}</span>
      </div>
    {:else}
      <EmptyState title={t("bomEmpty")} bare />
    {/each}
    <div class="flex items-center justify-between rounded-fc-lg bg-fc-component p-5 text-fc-lg font-semibold">
      <span>{t("total")}</span>
      <span>{money(bom.report.totals.total, bom.report.currency)}</span>
    </div>
  {/if}
</main>
