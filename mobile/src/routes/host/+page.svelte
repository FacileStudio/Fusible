<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { page } from "$app/state";
  import { Alert, Icon } from "@facile/muse";
  import FusePill from "../../components/FusePill.svelte";
  import { Current } from "$lib/current.svelte";
  import { Poller } from "$lib/poll.svelte";
  import { ICON } from "$lib/icons";
  import { useT } from "$lib/i18n";

  const t = useT();
  const cur = new Current();
  let poller = $state<Poller | null>(null);
  const id = $derived(page.url.searchParams.get("id") ?? "");

  onMount(async () => {
    await cur.load(id);
    if (!cur.host) return;
    poller = new Poller(cur.host);
    await poller.start();
  });
  onDestroy(() => poller?.stop());

  const summary = $derived(poller?.summary);
  const money = (a: number, c: string) => new Intl.NumberFormat(undefined, { style: "currency", currency: c }).format(a);
</script>

<main class="mx-auto flex max-w-xl flex-col gap-4 px-4 pt-8 pb-nav">
  <a href="/" class="text-fc-sm text-fc-fg-muted">{cur.host?.name ?? ""}</a>
  <h1 class="text-fc-2xl font-semibold">{summary?.project ?? ""}</h1>
  {#if poller?.error}
    <Alert tone="warning">{poller.error === "unpaired" ? t("unpaired") : t("offline")}</Alert>
  {/if}
  {#each summary?.boards ?? [] as board (board.name)}
    <a
      href={`/host/board?id=${encodeURIComponent(id)}&board=${encodeURIComponent(board.name)}`}
      class="flex flex-col gap-3 rounded-3xl bg-fc-surface p-5 focus-visible:outline-2 focus-visible:outline-fc-ring"
    >
      <span class="flex items-center justify-between gap-2">
        <span class="truncate text-fc-lg font-semibold">{board.name}</span>
        <FusePill blown={board.fuse_blown} />
      </span>
      <span class="text-fc-sm text-fc-fg-muted">
        {t("errors", { count: board.errors })} · {t("warnings", { count: board.warnings })}
      </span>
    </a>
  {/each}
  {#if summary}
    <div class="grid grid-cols-2 gap-3">
      <div class="flex flex-col gap-1 rounded-3xl bg-fc-surface p-5">
        <Icon icon={ICON.bom} size={18} class="text-fc-fg-muted" />
        <span class="text-fc-xs text-fc-fg-muted">{t("total")}</span>
        <span class="text-fc-lg font-semibold">
          {summary.bom_total ? money(summary.bom_total.amount, summary.bom_total.currency) : "-"}
        </span>
      </div>
      <div class="flex flex-col gap-1 rounded-3xl bg-fc-surface p-5">
        <Icon icon={ICON.bodies} size={18} class="text-fc-fg-muted" />
        <span class="text-fc-xs text-fc-fg-muted">{t("bodiesCount", { count: summary.bodies.length })}</span>
        <span class="text-fc-lg font-semibold">{summary.bodies.length}</span>
      </div>
    </div>
    <div class="rounded-3xl bg-fc-surface p-5">
      <span class="block text-fc-xs text-fc-fg-muted">{t("lastRun")}</span>
      <span class="text-fc-md font-medium">
        {summary.last_run ? `${summary.last_run.label ?? ""} ${summary.last_run.status ?? ""}` : t("noRun")}
      </span>
    </div>
  {/if}
  <p class="text-center text-fc-xs text-fc-fg-muted">{t("readOnly")}</p>
</main>
