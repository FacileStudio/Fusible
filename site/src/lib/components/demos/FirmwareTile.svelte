<script lang="ts">
  import { Bar, Roll, Stage, Tile, type Act } from "@facile/mise-en-scene";
  import type { DemoTile as Copy } from "$lib/content/types";

  let { copy }: { copy: Copy } = $props();

  let moved = $state(false);
  let generated = $state(false);

  function reset() {
    moved = false;
    generated = false;
  }

  function poster() {
    moved = true;
    generated = true;
  }

  async function script(act: Act) {
    await act.wait(1400);
    moved = true;
    await act.wait(1300);
    generated = true;
    await act.wait(2800);
    reset();
    await act.wait(700);
  }
</script>

<Tile title={copy.title} body={copy.text}>
  <Stage width={320} height={240} {script} {reset} {poster} label={copy.title}>
    {#snippet children()}
      <div class="ft-card" style="top:20px">
        <span class="ft-file">hardware/index.tsx</span>
        <Bar w="58%" />
        <code class="ft-code" class:ft-hit={moved && !generated}>LED="<Roll value={moved ? "IO14" : "IO12"} />"</code>
        <Bar w="42%" />
      </div>
      <div class="ft-card" style="top:132px">
        <span class="ft-file">firmware/src/pins.rs</span>
        <code class="ft-code" class:ft-hit={generated}>pub const LED: Gpio = <Roll value={generated ? "GPIO14" : "GPIO12"} />;</code>
        <Bar w="72%" />
        <Bar w="50%" />
      </div>
    {/snippet}
  </Stage>
</Tile>

<style>
  .ft-card {
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
  .ft-file {
    font-size: 11px;
    color: var(--color-fc-fg-muted);
  }
  .ft-code {
    width: fit-content;
    white-space: nowrap;
    font-family: var(--font-fc-mono);
    font-size: 12px;
    color: var(--color-fc-fg);
    border-radius: 6px;
    transition: color 0.25s ease;
  }
  .ft-hit {
    color: var(--color-fc-success);
  }
</style>
