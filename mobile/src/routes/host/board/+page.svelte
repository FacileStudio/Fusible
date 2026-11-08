<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { EmptyState, Tabs } from "@facile/muse";
  import { convertCircuitJsonToPcbSvg, convertCircuitJsonToSchematicSvg } from "circuit-to-svg";
  import FusePill from "../../../components/FusePill.svelte";
  import Viewer3D from "../../../components/Viewer3D.svelte";
  import ZoomPane from "../../../components/ZoomPane.svelte";
  import { get } from "$lib/client";
  import { fitSvg } from "$lib/svg";
  import { circuitToGlb } from "$lib/board3d";
  import { Current } from "$lib/current.svelte";
  import { type Summary } from "$lib/poll.svelte";
  import { useT, type Key } from "$lib/i18n";

  type Payload = { fuse_blown: boolean; circuit_json: unknown[] | null };
  type View = "pcb" | "schematic" | "3d";

  const t = useT();
  const cur = new Current();
  const id = $derived(page.url.searchParams.get("id") ?? "");
  let names = $state<string[]>([]);
  let board = $state("");
  let payload = $state<Payload | null>(null);
  let view = $state<View>("pcb");

  const views: { id: View; label: Key }[] = [
    { id: "pcb", label: "viewPcb" },
    { id: "schematic", label: "viewSchematic" },
    { id: "3d", label: "view3d" },
  ];

  async function open(name: string) {
    board = name;
    payload = await get<Payload>(cur.host!, `/api/circuit?board=${encodeURIComponent(name)}`);
  }

  onMount(async () => {
    await cur.load(id);
    if (!cur.host) return;
    const summary = await get<Summary>(cur.host, "/api/mobile/summary");
    names = summary.boards.map((b) => b.name);
    await open(page.url.searchParams.get("board") ?? names[0] ?? "");
  });

  function draw(kind: View) {
    const json = payload?.circuit_json;
    if (!json) return "";
    try {
      const raw =
        kind === "pcb" ? convertCircuitJsonToPcbSvg(json as never) : convertCircuitJsonToSchematicSvg(json as never);
      return fitSvg(raw);
    } catch {
      return "";
    }
  }

  const svg = $derived(view === "3d" ? "" : draw(view));
</script>

<main class="mx-auto flex max-w-xl flex-col gap-4 px-4 pt-8 pb-nav">
  <div class="flex items-center justify-between gap-2">
    <h1 class="truncate text-fc-2xl font-semibold">{board}</h1>
    {#if payload}<FusePill blown={payload.fuse_blown} />{/if}
  </div>
  {#if names.length > 1}
    <Tabs items={names.map((name) => ({ id: name, label: name }))} value={board} label={t("navBoards")} onChange={open} />
  {/if}
  <Tabs items={views.map((v) => ({ id: v.id, label: t(v.label) }))} value={view} label={t("navBoards")} onChange={(id) => (view = id as typeof view)} />
  {#if view === "3d" && payload?.circuit_json}
    {#key board}
      <Viewer3D load={() => circuitToGlb(payload!.circuit_json)} />
    {/key}
  {:else if svg}
    <ZoomPane><div class="size-full [&>svg]:size-full">{@html svg}</div></ZoomPane>
  {:else}
    <EmptyState title={t("nothingToShow")} bare />
  {/if}
</main>
