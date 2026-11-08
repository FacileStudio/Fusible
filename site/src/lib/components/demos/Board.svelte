<script lang="ts">
  let { blown, wide }: { blown: boolean; wide: boolean } = $props();

  const TRACES = [
    "M212 120 H262 V92 H300",
    "M212 150 H250 V176 H300",
    "M212 180 H238 V214 H296",
    "M150 196 V226 H110",
    "M120 150 H72",
    "M180 104 V62 H262",
  ];
  const PADS = [
    [300, 86], [300, 170], [296, 208], [262, 56], [100, 220], [64, 144], [330, 120], [330, 150],
  ];
</script>

<svg viewBox="0 0 400 280" class="board" aria-hidden="true">
  <rect x="24" y="20" width="352" height="240" rx="14" class="plate" />
  {#each TRACES as d (d)}<path {d} class="trace" />{/each}
  {#each PADS as [x, y] (`${x}-${y}`)}<rect {x} {y} width="14" height="10" rx="2" class="pad" />{/each}
  <rect x="120" y="96" width="92" height="100" rx="6" class="chip" />
  <rect x="24" y="118" width="40" height="44" rx="4" class="chip" />
  <path d="M64 134 H92 V116 H120" class="vbus" class:blown class:wide />
  <circle cx="340" cy="44" r="6" class="pad" />
  <circle cx="60" cy="236" r="6" class="pad" />
</svg>

<style>
  .board {
    width: 100%;
    height: 100%;
  }
  .plate {
    fill: var(--color-fc-surface);
  }
  .trace {
    fill: none;
    stroke: var(--color-fc-fg-muted);
    stroke-opacity: 0.45;
    stroke-width: 2;
  }
  .pad,
  .chip {
    fill: var(--color-fc-fg-muted);
    fill-opacity: 0.5;
  }
  .chip {
    fill-opacity: 0.3;
  }
  .vbus {
    fill: none;
    stroke: var(--color-fc-fg);
    stroke-width: 2;
    transition: stroke 0.25s ease, stroke-width 0.5s ease;
  }
  .vbus.blown {
    stroke: var(--color-fc-danger);
  }
  .vbus.wide {
    stroke-width: 7;
  }
</style>
