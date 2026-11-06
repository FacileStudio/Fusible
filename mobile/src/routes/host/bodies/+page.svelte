<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { Badge, EmptyState } from "@facile/muse";
  import Viewer3D from "../../../components/Viewer3D.svelte";
  import { get, getBuffer } from "$lib/client";
  import { Current } from "$lib/current.svelte";
  import type { Summary } from "$lib/poll.svelte";
  import { useT } from "$lib/i18n";

  const t = useT();
  const cur = new Current();
  const id = $derived(page.url.searchParams.get("id") ?? "");
  let summary = $state<Summary | null>(null);
  let picked = $state("");

  onMount(async () => {
    await cur.load(id);
    if (cur.host) summary = await get<Summary>(cur.host, "/api/mobile/summary");
  });

  const glb = (name: string) =>
    getBuffer(cur.host!, `/api/bodies/${encodeURIComponent(name)}.glb?rev=${summary?.revision ?? ""}`);
</script>

<main class="mx-auto flex max-w-xl flex-col gap-4 px-4 pt-8 pb-nav">
  <h1 class="text-fc-2xl font-semibold">{t("navBodies")}</h1>
  {#if summary && summary.bodies.length === 0}
    <EmptyState title={t("bodiesEmpty")} bare />
  {/if}
  {#each summary?.bodies ?? [] as body (body.name)}
    <button
      class="flex min-h-14 items-center justify-between gap-3 rounded-full px-7 py-5 text-left {picked === body.name
        ? 'bg-fc-accent text-fc-accent-fg'
        : 'bg-fc-surface'}"
      aria-pressed={picked === body.name}
      onclick={() => (picked = body.name)}
    >
      <span class="truncate text-fc-md font-medium">{body.name}</span>
      <Badge class="rounded-full">{body.kind}</Badge>
    </button>
  {/each}
  {#if picked}
    {#key picked}
      <Viewer3D load={() => glb(picked)} />
    {/key}
  {:else if summary?.bodies.length}
    <p class="text-center text-fc-sm text-fc-fg-muted">{t("pickBody")}</p>
  {/if}
</main>
