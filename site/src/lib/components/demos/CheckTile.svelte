<script lang="ts">
  import { Caret, Stage, Terminal, Tile, type Act } from "@facile/mise-en-scene";
  import type { DemoTile as Copy } from "$lib/content/types";

  let { copy }: { copy: Copy } = $props();

  type Line = { kind: "cmd" | "bad" | "out"; text: string };

  const SCRIPT: Line[] = [
    { kind: "cmd", text: "fusible check" },
    { kind: "bad", text: "× trace_current  VBUS" },
    { kind: "bad", text: "fuse blown, exit 1" },
    { kind: "cmd", text: "fusible export --preset fab" },
    { kind: "out", text: "nothing written" },
  ];

  let lines = $state<Line[]>([]);

  function reset() {
    lines = [];
  }

  function poster() {
    lines = [...SCRIPT];
  }

  async function script(act: Act) {
    await act.wait(600);
    for (const line of SCRIPT) {
      if (line.kind === "cmd") {
        lines.push({ kind: "cmd", text: "" });
        const at = lines.length - 1;
        await act.type(line.text, (v) => (lines[at].text = v), 55);
        await act.wait(450);
      } else {
        lines.push(line);
        await act.wait(800);
      }
    }
    await act.wait(2000);
    reset();
    await act.wait(400);
  }
</script>

<Tile title={copy.title} body={copy.text}>
  <Stage width={320} height={240} {script} {reset} {poster} label={copy.title}>
    {#snippet children()}
      <Terminal style="top:24px;left:24px;right:-40px;bottom:-24px">
        {#each lines as line, i (i)}
          <div class="ct-line" data-kind={line.kind}>
            {#if line.kind === "cmd"}<span class="ct-dim">$</span> {line.text}{#if i === lines.length - 1}<Caret />{/if}{:else}{line.text}{/if}
          </div>
        {/each}
        {#if lines.at(-1)?.kind !== "cmd"}
          <div class="ct-line"><span class="ct-dim">$</span> <Caret /></div>
        {/if}
      </Terminal>
    {/snippet}
  </Stage>
</Tile>

<style>
  .ct-line {
    white-space: nowrap;
    line-height: 20px;
  }
  .ct-dim,
  [data-kind="out"] {
    opacity: 0.55;
  }
  [data-kind="bad"] {
    color: oklch(0.72 0.17 25);
  }
</style>
