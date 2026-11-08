<script lang="ts">
  import { Stage, grow, type Act } from "@facile/mise-en-scene";
  import type { DemoTile as Copy } from "$lib/content/types";
  import DemoTile from "./DemoTile.svelte";

  let { copy }: { copy: Copy } = $props();

  const ROWS = [
    { name: "ESP32-S3", cost: 310 },
    { name: "AP2112K", cost: 24 },
    { name: "enclosure", cost: 69 },
  ];
  const FULL = ROWS.reduce((sum, row) => sum + row.cost, 0);

  let shown = $state(0);
  let cents = $state(0);

  const total = (n: number) => (n / 100).toFixed(2);

  function reset() {
    shown = 0;
    cents = 0;
  }

  function poster() {
    shown = ROWS.length;
    cents = FULL;
  }

  async function script(act: Act) {
    await act.wait(700);
    for (const row of ROWS) {
      shown += 1;
      await act.count(row.cost, (v) => (cents = FULL - ROWS.slice(shown - 1).reduce((s, r) => s + r.cost, 0) + v), 9);
      await act.wait(500);
    }
    await act.wait(2200);
    reset();
    await act.wait(500);
  }
</script>

<DemoTile title={copy.title} text={copy.text}>
  <Stage width={320} height={240} {script} {reset} {poster} label={copy.title}>
    {#snippet children()}
      <div class="bt-total">
        <span>{copy.label}</span>
        <b>{total(cents)} EUR</b>
      </div>
      <div class="bt-rows">
        {#each ROWS.slice(0, shown) as row (row.name)}
          <div class="bt-pad" transition:grow={{ duration: 300 }}>
            <div class="bt-row">
              <span class="bt-name">{row.name}</span>
              <span>{total(row.cost)}</span>
            </div>
          </div>
        {/each}
      </div>
    {/snippet}
  </Stage>
</DemoTile>

<style>
  .bt-total {
    position: absolute;
    top: 20px;
    left: 24px;
    right: 24px;
    display: flex;
    flex-direction: column;
    padding: 12px 16px;
    border-radius: 12px;
    background: var(--color-fc-accent);
    color: var(--color-fc-accent-fg);
    font-size: 11px;
  }
  .bt-total b {
    font-size: 20px;
    font-weight: 500;
  }
  .bt-rows {
    position: absolute;
    top: 92px;
    left: 24px;
    right: 24px;
  }
  .bt-pad {
    padding-bottom: 6px;
  }
  .bt-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 8px 14px;
    border-radius: 12px;
    background: var(--color-fc-surface);
    font-size: 12px;
    color: var(--color-fc-fg);
  }
  .bt-name {
    flex: 1;
  }
</style>
