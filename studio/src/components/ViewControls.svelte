<script lang="ts">
  import { Button, Tooltip } from "@facile/muse";
  import { MediaQuery } from "svelte/reactivity";
  import { launch } from "../lib/dispatch.svelte";
  import { mech, toggleMechanical } from "../lib/mechanical.svelte";
  import { studio } from "../lib/studio.svelte";
  import { cameraPreset, zoomBy, type Hosts } from "../lib/viewers";

  let { hosts, onfit }: { hosts: Hosts; onfit: () => void } = $props();

  const PRESETS = [
    { id: "TOP", label: "Top", icon: "solar:arrow-up-linear" },
    { id: "ISO", label: "Iso", icon: "solar:box-linear" },
    { id: "FRONT", label: "Front", icon: "solar:arrow-down-linear" },
    { id: "RESET", label: "Reset", icon: "solar:restart-linear" },
  ] as const;

  const flat = $derived(studio.view === "schematic" || studio.view === "pcb");
  const host = $derived(studio.view === "bom" ? undefined : hosts[studio.view]);
  const camera = $derived(studio.view === "3d" || studio.view === "mechanical" || studio.view === "assembly");
  const on = "bg-fc-component text-fc-fg";
  const wide = new MediaQuery("min-width: 48rem");
</script>

{#if studio.view !== "bom"}
  <div
    class="absolute bottom-4 left-1/2 z-10 inline-flex max-w-[calc(100%-24px)] -translate-x-1/2 gap-1 overflow-x-auto rounded-fc-pill bg-fc-surface p-1 shadow-lg"
    role="toolbar"
    aria-label="View controls"
  >
    {#if flat}
      <Tooltip text="Zoom out" describe={false} disabled={wide.current} class="contents">
        <Button variant="ghost" size="sm" icon="solar:magnifer-zoom-out-linear" aria-label="Zoom out" onclick={() => zoomBy(host, 120)}>
          <span class="max-md:hidden">Zoom out</span>
        </Button>
      </Tooltip>
      <Tooltip text="Zoom in" describe={false} disabled={wide.current} class="contents">
        <Button variant="ghost" size="sm" icon="solar:magnifer-zoom-in-linear" aria-label="Zoom in" onclick={() => zoomBy(host, -120)}>
          <span class="max-md:hidden">Zoom in</span>
        </Button>
      </Tooltip>
      <Tooltip text="Fit to view" describe={false} disabled={wide.current} class="contents">
        <Button variant="ghost" size="sm" icon="solar:restart-linear" aria-label="Fit to view" onclick={onfit}>
          <span class="max-md:hidden">Fit</span>
        </Button>
      </Tooltip>
    {/if}
    {#if camera}
      {#each PRESETS as preset (preset.id)}
        <Tooltip text={`${preset.label} view`} describe={false} disabled={wide.current} class="contents">
          <Button
            variant="ghost"
            size="sm"
            icon={preset.icon}
            aria-label={`${preset.label} view`}
            onclick={() => cameraPreset(studio.view, hosts, preset.id)}
          >
            <span class="max-md:hidden">{preset.label}</span>
          </Button>
        </Tooltip>
      {/each}
    {/if}
    {#if studio.view === "3d"}
      <Tooltip text="Enclosure" describe={false} disabled={wide.current} class="contents">
        <Button
          variant="ghost"
          size="sm"
          icon="solar:box-minimalistic-linear"
          aria-label="Enclosure"
          aria-pressed={mech.overlay}
          class={mech.overlay ? on : ""}
          onclick={() => toggleMechanical("overlay", hosts["3d"] ?? null)}
        >
          <span class="max-md:hidden">Enclosure</span>
        </Button>
      </Tooltip>
    {/if}
    {#if studio.view === "mechanical" && mech.lidAvailable}
      <Tooltip text="Lid" describe={false} disabled={wide.current} class="contents">
        <Button variant="ghost" size="sm" icon="solar:eye-linear" aria-label="Lid" aria-pressed={mech.lid} class={mech.lid ? on : ""} onclick={() => toggleMechanical("lid", null)}>
          <span class="max-md:hidden">Lid</span>
        </Button>
      </Tooltip>
      <Tooltip text="Explode" describe={false} disabled={wide.current} class="contents">
        <Button
          variant="ghost"
          size="sm"
          icon="solar:maximize-square-2-linear"
          aria-label="Explode"
          aria-pressed={mech.explode}
          class={mech.explode ? on : ""}
          onclick={() => toggleMechanical("explode", null)}
        >
          <span class="max-md:hidden">Explode</span>
        </Button>
      </Tooltip>
    {/if}
    {#if studio.view === "assembly"}
      <Tooltip text="Build assembly" describe={false} disabled={wide.current} class="contents">
        <Button variant="ghost" size="sm" icon="solar:layers-linear" aria-label="Build assembly" onclick={() => launch("assembly")}>
          <span class="max-md:hidden">Build assembly</span>
        </Button>
      </Tooltip>
    {/if}
  </div>
{/if}
