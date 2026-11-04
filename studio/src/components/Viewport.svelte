<script lang="ts">
  import { onMount, untrack } from "svelte";
  import BomView from "./BomView.svelte";
  import ViewControls from "./ViewControls.svelte";
  import { Tscircuit } from "../tscircuit";
  import { launch } from "../lib/dispatch.svelte";
  import { attachHover } from "../lib/hover";
  import { initMechanical, syncOverlay } from "../lib/mechanical.svelte";
  import { resendRevision } from "../lib/payload.svelte";
  import { selectComponent } from "../lib/selection.svelte";
  import { studio } from "../lib/studio.svelte";
  import { cameraPreset, parkViewers, refit, renderView, type Hosts } from "../lib/viewers";

  let hosts = $state<Hosts>({});

  const HOSTS = [
    ["schematic", "schematic-container"],
    ["pcb", "pcb-container"],
    ["3d", "three-cad"],
    ["mechanical", "mechanical-container"],
    ["assembly", "assembly-container"],
  ] as const;

  function render() {
    studio.status = renderView(studio.view, hosts, studio.circuit, selectComponent);
  }

  $effect(() => {
    void studio.circuit;
    void studio.theme;
    void studio.view;
    void hosts.schematic;
    untrack(() => {
      render();
      parkViewers(studio.view);
      syncOverlay(hosts["3d"] ?? null);
      if (studio.view === "3d") cameraPreset("3d", hosts, "ISO");
      if (studio.view === "schematic") setTimeout(refit, 10);
    });
  });

  $effect(() => {
    void studio.sidebarOpen;
    void studio.narrow;
    requestAnimationFrame(refit);
  });

  $effect(() => {
    const { schematic, pcb } = hosts;
    const stops = [schematic && attachHover(schematic, "schematic"), pcb && attachHover(pcb, "pcb")];
    return () => stops.forEach((stop) => stop?.());
  });

  onMount(() => {
    Tscircuit.setAssemblyBuildHandler(() => launch("assembly"));
    resendRevision();
    return initMechanical(hosts["3d"] ?? null);
  });
</script>

<section class="relative min-w-0 flex-1 overflow-hidden rounded-3xl bg-fc-component" aria-label="Viewport">
  {#each HOSTS as [name, id] (name)}
    <div {id} class="view-host" class:hidden={studio.view !== name} bind:this={hosts[name]}></div>
  {/each}
  {#if studio.view === "bom"}
    <BomView />
  {/if}
  <ViewControls {hosts} onfit={render} />
</section>
