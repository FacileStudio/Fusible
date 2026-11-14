<script lang="ts">
  import { Roll, Stage, Tile, type Act } from "@facile/mise-en-scene";
  import { Icon } from "@facile/muse";
  import type { DemoTile as Copy } from "$lib/content/types";

  let { copy }: { copy: Copy } = $props();

  const NETS = [
    { from: [100, 40], to: [196, 30], path: "M100 40 H140 L150 30 H196" },
    { from: [100, 60], to: [196, 56], path: "M100 60 H144 L148 56 H196" },
    { from: [100, 80], to: [196, 84], path: "M100 80 H144 L148 84 H196" },
    { from: [100, 100], to: [196, 110], path: "M100 100 H140 L150 110 H196" },
  ];

  let routed = $state(0);

  function reset() {
    routed = 0;
  }

  function poster() {
    routed = NETS.length;
  }

  async function script(act: Act) {
    await act.wait(1000);
    await act.count(NETS.length, (v) => (routed = v), 480);
    await act.wait(2600);
    reset();
    await act.wait(700);
  }
</script>

<Tile title={copy.title} body={copy.text}>
  <Stage width={320} height={240} {script} {reset} {poster} label={copy.title}>
    {#snippet children()}
      <div class="rt-plate">
        <svg viewBox="0 0 272 140" aria-hidden="true">
          <rect x="44" y="26" width="56" height="88" rx="6" class="rt-part" />
          <rect x="196" y="18" width="40" height="104" rx="5" class="rt-part" />
          {#each NETS as net, i (net.path)}
            <path
              d="M{net.from[0]} {net.from[1]} L{net.to[0]} {net.to[1]}"
              class="rt-rat"
              class:gone={i < routed}
            />
            <path d={net.path} pathLength="1" class="rt-trace" class:on={i < routed} />
            <rect x={net.from[0] - 6} y={net.from[1] - 4} width="10" height="8" rx="2" class="rt-pad" />
            <rect x={net.to[0] - 4} y={net.to[1] - 4} width="10" height="8" rx="2" class="rt-pad" />
          {/each}
        </svg>
      </div>
      <div class="rt-pill" class:done={routed === NETS.length}>
        <Icon icon="solar:check-circle-linear" size={14} />
        <code>fusible route</code>
        <Roll value="{routed}/{NETS.length}" />
      </div>
    {/snippet}
  </Stage>
</Tile>

<style>
  .rt-plate {
    position: absolute;
    top: 20px;
    left: 24px;
    right: 24px;
    height: 148px;
    padding: 4px 0;
    border-radius: 12px;
    background: var(--color-fc-surface);
  }
  .rt-plate svg {
    width: 100%;
    height: 100%;
  }
  .rt-part {
    fill: var(--color-fc-fg-muted);
    fill-opacity: 0.25;
  }
  .rt-pad {
    fill: var(--color-fc-fg-muted);
    fill-opacity: 0.6;
  }
  .rt-rat {
    stroke: var(--color-fc-fg-muted);
    stroke-width: 1.2;
    stroke-dasharray: 3 3;
    transition: opacity 0.3s ease;
  }
  .rt-rat.gone {
    opacity: 0;
  }
  .rt-trace {
    fill: none;
    stroke: var(--color-fc-success);
    stroke-width: 3;
    stroke-linejoin: round;
    stroke-linecap: round;
    stroke-dasharray: 1;
    stroke-dashoffset: 1;
    transition: stroke-dashoffset 0.45s ease;
  }
  .rt-trace.on {
    stroke-dashoffset: 0;
  }
  .rt-pill {
    position: absolute;
    top: 186px;
    left: 24px;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    height: 32px;
    padding: 0 14px;
    border-radius: 999px;
    background: var(--color-fc-surface);
    color: var(--color-fc-fg-muted);
    font-size: 12px;
    white-space: nowrap;
    transition: color 0.25s ease;
  }
  .rt-pill code {
    font-family: var(--font-fc-mono);
    color: var(--color-fc-fg);
  }
  .rt-pill.done {
    color: var(--color-fc-success);
  }
</style>
