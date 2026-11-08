<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { Badge, EmptyState, Icon } from "@facile/muse";
  import { get } from "$lib/client";
  import { Current } from "$lib/current.svelte";
  import type { Summary } from "$lib/poll.svelte";
  import { ICON } from "$lib/icons";
  import { useT } from "$lib/i18n";

  type Finding = {
    rule: string;
    message: string;
    help?: string;
    severity?: string;
    standard?: string;
    source_url?: string;
    board: string;
  };

  const t = useT();
  const cur = new Current();
  const id = $derived(page.url.searchParams.get("id") ?? "");
  let findings = $state<Finding[]>([]);
  let loaded = $state(false);

  onMount(async () => {
    await cur.load(id);
    if (!cur.host) return;
    const summary = await get<Summary>(cur.host, "/api/mobile/summary");
    for (const b of summary.boards) {
      const p = await get<{ diagnostics: Omit<Finding, "board">[] }>(
        cur.host,
        `/api/circuit?board=${encodeURIComponent(b.name)}`,
      );
      findings.push(...p.diagnostics.map((d) => ({ ...d, board: b.name })));
    }
    loaded = true;
  });
</script>

<main class="mx-auto flex max-w-xl flex-col gap-4 px-4 pt-8 pb-nav">
  <h1 class="text-fc-2xl font-semibold">{t("navChecks")}</h1>
  {#each findings as f, i (i)}
    <article class="flex flex-col gap-2 rounded-fc-lg bg-fc-surface p-5">
      <div class="flex flex-wrap items-center gap-2">
        <Badge tone={f.severity === "error" ? "danger" : "warning"} class="rounded-fc-pill">{f.rule}</Badge>
        <span class="text-fc-xs text-fc-fg-muted">{f.board}</span>
      </div>
      <p class="text-fc-md">{f.message}</p>
      {#if f.help}<p class="text-fc-sm text-fc-fg-muted">{f.help}</p>{/if}
      {#if f.standard && f.source_url}
        <a
          href={f.source_url}
          target="_blank"
          rel="noopener noreferrer"
          class="inline-flex min-h-11 items-center gap-2 self-start rounded-fc-pill bg-fc-component px-4 text-fc-sm focus-visible:outline-2 focus-visible:outline-fc-ring"
        >
          <Icon icon={ICON.link} size={14} />
          {f.standard}
        </a>
      {:else if f.standard}
        <span class="self-start rounded-fc-pill bg-fc-component px-4 py-2 text-fc-sm">{f.standard}</span>
      {/if}
    </article>
  {:else}
    {#if loaded}<EmptyState icon={ICON.ok} title={t("noChecks")} bare />{/if}
  {/each}
  <p class="text-center text-fc-xs text-fc-fg-muted">{t("preCheck")}</p>
</main>
