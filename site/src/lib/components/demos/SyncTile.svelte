<script lang="ts">
  import { Bar, Roll, Stage, Tile, type Act } from "@facile/mise-en-scene";
  import type { DemoTile as Copy } from "$lib/content/types";

  let { copy }: { copy: Copy } = $props();

  let moved = $state(false);
  let synced = $state(false);

  function reset() {
    moved = false;
    synced = false;
  }

  function poster() {
    moved = true;
    synced = true;
  }

  async function script(act: Act) {
    await act.wait(1200);
    moved = true;
    await act.wait(1500);
    synced = true;
    await act.wait(2800);
    reset();
    await act.wait(700);
  }
</script>

<Tile title={copy.title} body={copy.text}>
  <Stage width={320} height={240} {script} {reset} {poster} label={copy.title}>
    {#snippet children()}
      <div class="sy-card" style="top:20px">
        <span class="sy-file">board.kicad_pcb</span>
        <div class="sy-board">
          <span class="sy-part" class:moved>U2</span>
        </div>
      </div>
      <div class="sy-card" style="top:132px">
        <span class="sy-file">manual-edits.json</span>
        <code class="sy-code" class:sy-hit={synced}>"x": <Roll value={synced ? "18.5" : "12.0"} />,</code>
        <Bar w="64%" />
        <Bar w="40%" />
      </div>
    {/snippet}
  </Stage>
</Tile>

<style>
  .sy-card {
    position: absolute;
    left: 24px;
    right: 24px;
    display: flex;
    flex-direction: column;
    gap: 7px;
    padding: 12px 14px;
    border-radius: 12px;
    background: var(--color-fc-surface);
  }
  .sy-file {
    font-size: 11px;
    color: var(--color-fc-fg-muted);
  }
  .sy-board {
    position: relative;
    height: 48px;
    border-radius: 8px;
    background: var(--color-fc-field);
  }
  .sy-part {
    position: absolute;
    top: 8px;
    left: 16px;
    display: grid;
    place-items: center;
    width: 48px;
    height: 32px;
    border-radius: 6px;
    background: var(--color-fc-fg-muted);
    color: var(--color-fc-surface);
    font-family: var(--font-fc-mono);
    font-size: 11px;
    transition: transform 0.7s cubic-bezier(0.25, 0.8, 0.25, 1);
  }
  .sy-part.moved {
    transform: translateX(96px);
  }
  .sy-code {
    width: fit-content;
    white-space: nowrap;
    font-family: var(--font-fc-mono);
    font-size: 12px;
    color: var(--color-fc-fg);
    transition: color 0.25s ease;
  }
  .sy-hit {
    color: var(--color-fc-success);
  }
</style>
