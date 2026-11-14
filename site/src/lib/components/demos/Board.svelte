<script lang="ts">
  let { blown, wide, drawn = true }: { blown: boolean; wide: boolean; drawn?: boolean } = $props();

  const TRACES = [
    "M212 120 H254 Q262 120 262 112 V100 Q262 92 270 92 H300",
    "M212 150 H242 Q250 150 250 158 V168 Q250 176 258 176 H300",
    "M212 180 H230 Q238 180 238 188 V206 Q238 214 246 214 H296",
    "M150 196 V218 Q150 226 142 226 H114",
    "M120 150 H78",
    "M180 96 V70 Q180 62 188 62 H262",
  ];
  const PADS = [
    [300, 86], [300, 170], [296, 208], [262, 56], [100, 220], [64, 144], [330, 120], [330, 150],
  ];
</script>

<svg viewBox="0 0 400 280" class="board" aria-hidden="true">
  <rect x="24" y="20" width="352" height="240" rx="14" class="plate" />
  {#each TRACES as d (d)}<path {d} class="trace" />{/each}
  <path d="M64 134 H86 Q92 134 92 128 V122 Q92 116 98 116 H120" class="vbus" class:blown class:wide class:hidden={!drawn} />
  {#each PADS as [x, y] (`${x}-${y}`)}
    <rect {x} {y} width="14" height="10" rx="2" class="pad-base" />
    <rect {x} {y} width="14" height="10" rx="2" class="pad" />
  {/each}
  <circle cx="340" cy="44" r="6" class="pad" />
  <circle cx="60" cy="236" r="6" class="pad" />
  <rect x="120" y="96" width="92" height="100" rx="6" class="chip-base" />
  <rect x="120" y="96" width="92" height="100" rx="6" class="chip" />
  <rect x="24" y="118" width="40" height="44" rx="4" class="chip-base" />
  <rect x="24" y="118" width="40" height="44" rx="4" class="chip" />
</svg>

<style>
  .board {
    width: 100%;
    height: 100%;
  }
  .plate,
  .pad-base,
  .chip-base {
    fill: var(--color-fc-surface);
  }
  .trace {
    fill: none;
    stroke: var(--color-fc-fg-muted);
    stroke-opacity: 0.45;
    stroke-width: 2;
    stroke-linejoin: round;
    stroke-linecap: round;
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
    stroke-linejoin: round;
    stroke-linecap: round;
    transition: stroke 0.25s ease, stroke-width 0.5s ease, opacity 0.35s ease;
  }
  .vbus.hidden {
    opacity: 0;
  }
  .vbus.blown {
    stroke: var(--color-fc-danger);
  }
  .vbus.wide {
    stroke-width: 7;
  }
</style>
