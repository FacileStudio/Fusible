<script lang="ts">
  import { tick } from "svelte";
  import { Bar, Caret, Cursor, Roll, Stage, Window, cursorAt, grow, type Act } from "@facile/mise-en-scene";
  import { Icon } from "@facile/muse";
  import type { HeroDemoCopy } from "$lib/content/types";
  import Board from "./Board.svelte";

  let { copy }: { copy: HeroDemoCopy } = $props();

  const FILES = ["gerbers.zip", "board.step", "pins.rs"];
  const LINE = [
    { text: "<", tone: "text-fc-fg-muted" },
    { text: "trace", tone: "text-fc-info font-medium" },
    { text: " " },
    { text: "net", tone: "text-fc-admin font-medium" },
    { text: "=", tone: "text-fc-fg-muted" },
    { text: '"VBUS"', tone: "text-fc-success" },
    { text: " " },
    { text: "width", tone: "text-fc-admin font-medium" },
    { text: "=", tone: "text-fc-fg-muted" },
    { text: '"', tone: "text-fc-success" },
    { value: true, text: "0.2mm" },
    { text: '"', tone: "text-fc-success" },
    { text: " " },
    { text: "/>", tone: "text-fc-fg-muted" },
  ];
  const STARTS = LINE.map((_, i) => LINE.slice(0, i).reduce((sum, token) => sum + token.text.length, 0));
  const FULL = LINE.reduce((sum, token) => sum + token.text.length, 0);

  let blown = $state(false);
  let wide = $state(false);
  let width = $state("0.2");
  let refused = $state(false);
  let files = $state(0);
  let typed = $state(0);
  let cursor = $state(cursorAt());
  let check = $state<HTMLElement>();
  let exportButton = $state<HTMLElement>();
  let value = $state<HTMLElement>();

  function reset() {
    blown = false;
    wide = false;
    width = "0.2";
    refused = false;
    files = 0;
    typed = 0;
    cursor.visible = false;
  }

  function poster() {
    wide = true;
    width = "0.8";
    files = FILES.length;
    typed = FULL;
  }

  function shown(i: number): string {
    const token = LINE[i];
    const text = token.value ? `${width}mm` : token.text;
    return text.slice(0, Math.max(0, typed - STARTS[i]));
  }

  async function press(act: Act, target: HTMLElement | undefined, onPress?: () => void) {
    await act.move(cursor, act.point(target, 0.7, 0.72));
    await act.click(cursor, onPress);
  }

  async function script(act: Act) {
    await act.wait(500);
    await act.count(FULL, (v) => (typed = v), 45);
    await act.wait(600);
    await tick();
    cursor.visible = true;
    await press(act, check);
    blown = true;
    await act.wait(1400);
    await press(act, exportButton);
    refused = true;
    await act.wait(1500);
    await press(act, value);
    await act.wait(250);
    width = "";
    await act.wait(250);
    await act.type("0.8", (v) => (width = v), 140);
    await act.wait(500);
    wide = true;
    blown = false;
    refused = false;
    await act.wait(1100);
    await press(act, exportButton);
    await act.count(FILES.length, (v) => (files = v), 300);
    await act.wait(2300);
    reset();
    await act.wait(600);
  }
</script>

{#snippet pill()}
  <span class="hd-pill" class:hd-blown={blown}>
    <Icon icon={blown ? "solar:danger-triangle-bold-duotone" : "solar:shield-check-bold-duotone"} size={14} />
    <Roll value={blown ? copy.blown : copy.intact} />
  </span>
{/snippet}

{#snippet code()}
  <span class="hd-file">hardware/index.tsx</span>
  <code
    >{#each LINE as token, i (i)}{#if token.value}{#if typed > STARTS[i]}<span class="text-fc-success"
            ><span bind:this={value} class="hd-value">{shown(i)}</span></span
          >{/if}{:else}<span class={token.tone}>{shown(i)}</span>{/if}{/each}{#if typed < FULL}<Caret />{/if}</code
  >
{/snippet}

{#snippet panel()}
  <div class="hd-actions">
    <button bind:this={check} type="button" tabindex="-1">{copy.check}</button>
    <button bind:this={exportButton} type="button" tabindex="-1">{copy.export}</button>
  </div>
  <div class="hd-list">
    {#if blown}
      <div class="hd-pad" transition:grow={{ duration: 320 }}>
        <div class="hd-card">
          <b><Icon icon="solar:danger-triangle-bold-duotone" size={14} />trace_current</b>
          <p>{copy.violation}</p>
          <span class="hd-chip">IPC-2152</span>
        </div>
      </div>
    {/if}
    {#if refused}
      <div class="hd-pad" transition:grow={{ duration: 280 }}><div class="hd-refused">{copy.refused}</div></div>
    {/if}
    {#each FILES.slice(0, files) as file (file)}
      <div class="hd-pad" transition:grow={{ duration: 240 }}>
        <div class="hd-file-row"><Icon icon="solar:check-circle-linear" size={14} />{file}</div>
      </div>
    {/each}
    {#if !blown && !refused && files === 0}
      <div class="hd-idle"><Bar w="70%" /><Bar w="46%" /></div>
    {/if}
  </div>
{/snippet}

<Stage width={960} height={580} compact={{ below: 640, width: 358, height: 600 }} {script} {reset} {poster} label={copy.label}>
  {#snippet children({ compact })}
    {#if compact}
      <Window title="fieldcam" style="top:12px;left:12px;width:334px;height:576px">
        <div class="hd-compact">
          {@render pill()}
          <div class="hd-board"><Board {blown} {wide} drawn={typed >= FULL} /></div>
          <div class="hd-inline">{@render code()}</div>
          {@render panel()}
        </div>
      </Window>
    {:else}
      <Window title="fieldcam" style="top:24px;left:40px;width:880px;height:532px">
        {#snippet trailing()}{@render pill()}{/snippet}
        <div class="hd-body">
          <div class="hd-view">
            <div class="hd-tabs">
              {#each copy.views as view, i (view)}<span class:hd-on={i === 1}>{view}</span>{/each}
            </div>
            <div class="hd-board"><Board {blown} {wide} drawn={typed >= FULL} /></div>
            <div class="hd-inline">{@render code()}</div>
          </div>
          <aside class="hd-side">
            <span class="hd-title">{copy.checks}</span>
            {@render panel()}
          </aside>
        </div>
      </Window>
    {/if}
    <Cursor {cursor} />
  {/snippet}
</Stage>

<style>
  .hd-body {
    display: grid;
    flex: 1;
    grid-template-columns: minmax(0, 1fr) 272px;
    gap: 8px;
    min-height: 0;
    padding: 8px;
  }
  .hd-compact {
    display: flex;
    flex: 1;
    flex-direction: column;
    gap: 8px;
    min-height: 0;
    padding: 10px;
    color: var(--color-fc-fg);
  }
  .hd-compact .hd-pill {
    align-self: flex-start;
  }
  .hd-view {
    display: flex;
    flex-direction: column;
    gap: 8px;
    min-height: 0;
    padding: 8px;
    border-radius: 12px;
    background: var(--color-fc-component);
  }
  .hd-tabs {
    display: flex;
    gap: 2px;
    font-size: 12px;
    color: var(--color-fc-fg-muted);
  }
  .hd-tabs span {
    padding: 5px 12px;
    border-radius: 999px;
  }
  .hd-on {
    background: var(--color-fc-accent);
    color: var(--color-fc-accent-fg);
  }
  .hd-board {
    flex: 1;
    min-height: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 8px;
    background: var(--color-fc-page);
  }
  .hd-compact .hd-board {
    flex: none;
    height: 190px;
    background: var(--color-fc-component);
  }
  .hd-inline {
    display: flex;
    flex-direction: column;
    gap: 3px;
    padding: 8px 12px;
    border-radius: 8px;
    background: var(--color-fc-surface);
    font-size: 12px;
    color: var(--color-fc-fg);
  }
  .hd-inline code {
    font-family: var(--font-fc-mono);
    white-space: nowrap;
  }
  .hd-compact .hd-inline code {
    font-size: 10px;
  }
  .hd-file,
  .hd-title {
    font-size: 11px;
    color: var(--color-fc-fg-muted);
  }
  .hd-title {
    padding: 4px;
    font-size: 13px;
    font-weight: 500;
    color: var(--color-fc-fg);
  }
  .hd-value {
    padding: 0 2px;
    border-radius: 4px;
    background: var(--color-fc-field);
    color: var(--color-fc-success);
  }
  .hd-side {
    display: flex;
    flex-direction: column;
    gap: 8px;
    min-height: 0;
    padding: 12px;
    border-radius: 12px;
    background: var(--color-fc-component);
    color: var(--color-fc-fg);
  }
  .hd-pill {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    height: 24px;
    padding: 0 10px;
    border-radius: 999px;
    background: var(--color-fc-surface);
    color: var(--color-fc-fg);
    font-size: 11px;
    font-weight: 500;
    white-space: nowrap;
    transition: background 0.25s ease, color 0.25s ease;
  }
  .hd-blown {
    background: var(--color-fc-danger);
    color: var(--color-fc-danger-fg);
  }
  .hd-actions {
    display: flex;
    gap: 4px;
  }
  .hd-actions button {
    height: 30px;
    padding: 0 14px;
    border-radius: 999px;
    background: var(--color-fc-surface);
    color: var(--color-fc-fg);
    font-size: 12px;
  }
  .hd-list {
    min-height: 0;
    overflow: hidden;
  }
  .hd-pad {
    padding-bottom: 6px;
  }
  .hd-card {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 6px;
    padding: 10px 12px;
    border-radius: 12px;
    background: var(--color-fc-surface);
    font-size: 12px;
  }
  .hd-card b {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    font-weight: 500;
    color: var(--color-fc-danger);
  }
  .hd-card p {
    margin: 0;
    line-height: 1.45;
  }
  .hd-chip {
    padding: 2px 10px;
    border-radius: 999px;
    background: var(--color-fc-field);
    font-size: 11px;
  }
  .hd-refused {
    padding: 8px 12px;
    border-radius: 12px;
    background: color-mix(in oklch, var(--color-fc-danger) 12%, transparent);
    color: var(--color-fc-danger);
    font-size: 12px;
  }
  .hd-file-row {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    border-radius: 12px;
    background: var(--color-fc-surface);
    font-family: var(--font-fc-mono);
    font-size: 12px;
  }
  .hd-file-row :global(svg) {
    color: var(--color-fc-success);
  }
  .hd-idle {
    display: grid;
    gap: 8px;
    padding: 10px 4px;
  }
</style>
