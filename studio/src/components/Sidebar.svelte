<script lang="ts">
  import { onMount } from "svelte";
  import { Drawer, Tabs } from "@facile/muse";
  import Inspector from "./panes/Inspector.svelte";
  import Components from "./panes/Components.svelte";
  import Nets from "./panes/Nets.svelte";
  import Checks from "./panes/Checks.svelte";
  import Rules from "./panes/Rules.svelte";
  import Bodies from "./panes/Bodies.svelte";
  import Runs from "./panes/Runs.svelte";
  import Pins from "./panes/Pins.svelte";
  import { catalog, loadBodies, loadRules } from "../lib/catalog.svelte";
  import { studio } from "../lib/studio.svelte";
  import type { PaneName } from "../lib/types";

  const ALL = [
    { id: "inspector", label: "Inspect", icon: "solar:cursor-linear" },
    { id: "components", label: "Parts", icon: "solar:cpu-linear" },
    { id: "nets", label: "Nets", icon: "solar:share-circle-linear" },
    { id: "checks", label: "Checks", icon: "solar:checklist-minimalistic-linear" },
    { id: "rules", label: "Rules", icon: "solar:shield-check-linear" },
    { id: "bodies", label: "Bodies", icon: "solar:box-minimalistic-linear" },
    { id: "runs", label: "Runs", icon: "solar:play-circle-linear" },
    { id: "pins", label: "Pins", icon: "solar:plug-circle-linear" },
  ];

  const items = $derived(ALL.filter((tab) => tab.id !== "rules" || catalog.rulesAvailable));

  onMount(() => {
    void loadRules();
  });

  $effect(() => {
    void studio.revision;
    void loadBodies();
  });
</script>

{#snippet content()}
  <div class="flex min-h-0 flex-1 flex-col gap-2">
    <Tabs {items} value={studio.pane} label="Panels" class="shrink-0 border-0" onChange={(id) => (studio.pane = id as PaneName)} />
    <div class="min-h-0 flex-1 overflow-y-auto" role="tabpanel">
      {#if studio.pane === "inspector"}<Inspector />
      {:else if studio.pane === "components"}<Components />
      {:else if studio.pane === "nets"}<Nets />
      {:else if studio.pane === "checks"}<Checks />
      {:else if studio.pane === "rules"}<Rules />
      {:else if studio.pane === "bodies"}<Bodies />
      {:else if studio.pane === "runs"}<Runs />
      {:else}<Pins />{/if}
    </div>
  </div>
{/snippet}

{#if studio.narrow}
  <Drawer bind:open={studio.sheetOpen} title="Panel" showClose>
    <div class="flex h-[60dvh] flex-col">{@render content()}</div>
  </Drawer>
{:else if studio.sidebarOpen}
  <aside class="flex w-90 shrink-0 flex-col rounded-3xl bg-fc-surface p-3" aria-label="Panel">
    {@render content()}
  </aside>
{/if}
