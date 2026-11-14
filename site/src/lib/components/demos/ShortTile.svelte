<script lang="ts">
  import { Stage, Tile, grow, type Act } from "@facile/mise-en-scene";
  import { Icon } from "@facile/muse";
  import type { DemoTile as Copy } from "$lib/content/types";

  let { copy }: { copy: Copy } = $props();

  let drawn = $state(false);
  let flagged = $state(false);

  function reset() {
    drawn = false;
    flagged = false;
  }

  function poster() {
    drawn = true;
    flagged = true;
  }

  async function script(act: Act) {
    await act.wait(900);
    drawn = true;
    await act.wait(1100);
    flagged = true;
    await act.wait(2600);
    reset();
    await act.wait(600);
  }
</script>

<Tile title={copy.title} body={copy.text}>
  <Stage width={320} height={240} {script} {reset} {poster} label={copy.title}>
    {#snippet children()}
      <div class="st-plate">
        <svg viewBox="0 0 272 120" aria-hidden="true">
          <text x="14" y="18" class="st-net">3V3</text>
          <text x="14" y="112" class="st-net">GND</text>
          <path d="M44 14 H258" class="st-wire" />
          <path d="M44 108 H258" class="st-wire" />
          <path d="M76 14 V34 M76 88 V108" class="st-wire" />
          <rect x="70" y="34" width="12" height="54" rx="3" class="st-part" />
          <rect x="118" y="38" width="52" height="46" rx="6" class="st-part" />
          <text x="144" y="65" class="st-ref">U1</text>
          <path d="M144 14 V38 M132 84 V108" class="st-wire" />
          <path d="M212 14 V108" pathLength="1" class="st-short" class:drawn class:flagged />
          <circle cx="212" cy="14" r="3.5" class="st-dot" class:drawn class:flagged />
          <circle cx="212" cy="108" r="3.5" class="st-dot" class:drawn class:flagged />
        </svg>
      </div>
      <div class="st-list">
        {#if flagged}
          <div transition:grow={{ duration: 320 }}>
            <div class="st-card">
              <b><Icon icon="solar:danger-triangle-bold-duotone" size={14} />shorted_nets</b>
              <code>3V3 = GND</code>
            </div>
          </div>
        {/if}
      </div>
    {/snippet}
  </Stage>
</Tile>

<style>
  .st-plate {
    position: absolute;
    top: 20px;
    left: 24px;
    right: 24px;
    height: 132px;
    padding: 6px 0;
    border-radius: 12px;
    background: var(--color-fc-surface);
  }
  .st-plate svg {
    width: 100%;
    height: 100%;
  }
  .st-net,
  .st-ref {
    font-family: var(--font-fc-mono);
    font-size: 10px;
    fill: var(--color-fc-fg-muted);
  }
  .st-ref {
    text-anchor: middle;
  }
  .st-wire,
  .st-short {
    fill: none;
    stroke: var(--color-fc-fg-muted);
    stroke-opacity: 0.6;
    stroke-width: 2;
    stroke-linecap: round;
  }
  .st-part {
    fill: var(--color-fc-fg-muted);
    fill-opacity: 0.25;
  }
  .st-short {
    stroke: var(--color-fc-fg);
    stroke-opacity: 1;
    stroke-dasharray: 1;
    stroke-dashoffset: 1;
    transition: stroke-dashoffset 0.7s ease, stroke 0.25s ease;
  }
  .st-short.drawn {
    stroke-dashoffset: 0;
  }
  .st-short.flagged,
  .st-dot.flagged {
    stroke: var(--color-fc-danger);
    fill: var(--color-fc-danger);
  }
  .st-dot {
    fill: var(--color-fc-fg);
    opacity: 0;
    transition: fill 0.25s ease, opacity 0.2s ease 0.5s;
  }
  .st-dot.drawn {
    opacity: 1;
  }
  .st-short.flagged {
    fill: none;
  }
  .st-list {
    position: absolute;
    top: 164px;
    left: 24px;
    right: 24px;
  }
  .st-card {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    padding: 10px 12px;
    border-radius: 12px;
    background: var(--color-fc-surface);
    font-size: 12px;
  }
  .st-card b {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    font-weight: 500;
    color: var(--color-fc-danger);
  }
  .st-card code {
    padding: 2px 10px;
    border-radius: 999px;
    background: var(--color-fc-field);
    font-family: var(--font-fc-mono);
    font-size: 11px;
    color: var(--color-fc-fg);
  }
</style>
