<script lang="ts">
  import { Button } from "@facile/muse";
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
</script>

{#if studio.view !== "bom"}
  <div
    class="absolute bottom-4 left-1/2 z-10 inline-flex max-w-[calc(100%-24px)] -translate-x-1/2 gap-1 overflow-x-auto rounded-full bg-fc-surface p-1"
    role="toolbar"
    aria-label="View controls"
  >
    {#if flat}
      <Button variant="ghost" size="sm" icon="solar:magnifer-zoom-out-linear" aria-label="Zoom out" onclick={() => zoomBy(host, 120)}>
        <span class="max-md:hidden">Zoom out</span>
      </Button>
      <Button variant="ghost" size="sm" icon="solar:magnifer-zoom-in-linear" aria-label="Zoom in" onclick={() => zoomBy(host, -120)}>
        <span class="max-md:hidden">Zoom in</span>
      </Button>
      <Button variant="ghost" size="sm" icon="solar:restart-linear" aria-label="Fit to view" onclick={onfit}>
        <span class="max-md:hidden">Fit</span>
      </Button>
    {/if}
    {#if camera}
      {#each PRESETS as preset (preset.id)}
        <Button
          variant="ghost"
          size="sm"
          icon={preset.icon}
          aria-label={`${preset.label} view`}
          onclick={() => cameraPreset(studio.view, hosts, preset.id)}
        >
          <span class="max-md:hidden">{preset.label}</span>
        </Button>
      {/each}
    {/if}
    {#if studio.view === "3d"}
      <Button
        variant="ghost"
        size="sm"
        icon="solar:box-minimalistic-linear"
        aria-pressed={mech.overlay}
        class={mech.overlay ? on : ""}
        onclick={() => toggleMechanical("overlay", hosts["3d"] ?? null)}
      >
        <span class="max-md:hidden">Enclosure</span>
      </Button>
    {/if}
    {#if studio.view === "mechanical" && mech.lidAvailable}
      <Button variant="ghost" size="sm" icon="solar:eye-linear" aria-pressed={mech.lid} class={mech.lid ? on : ""} onclick={() => toggleMechanical("lid", null)}>
        <span class="max-md:hidden">Lid</span>
      </Button>
      <Button
        variant="ghost"
        size="sm"
        icon="solar:maximize-square-2-linear"
        aria-pressed={mech.explode}
        class={mech.explode ? on : ""}
        onclick={() => toggleMechanical("explode", null)}
      >
        <span class="max-md:hidden">Explode</span>
      </Button>
    {/if}
    {#if studio.view === "assembly"}
      <Button variant="ghost" size="sm" icon="solar:layers-linear" onclick={() => launch("assembly")}>
        <span class="max-md:hidden">Build assembly</span>
      </Button>
    {/if}
  </div>
{/if}
