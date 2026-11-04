<script lang="ts">
  import { Button, EmptyState } from "@facile/muse";
  import { launch } from "../../lib/dispatch.svelte";
  import { isBusy, runs } from "../../lib/runs.svelte";

  const busy = $derived(isBusy());
  const log = $derived(runs.list.find((run) => run.id === runs.active)?.lines.join("\n") ?? "");
  const STATUS = { running: "Running", ok: "OK", fuse_blown: "Fuse blown", error: "Failed" };
  const TONE = { running: "text-fc-info", ok: "text-fc-success", fuse_blown: "text-fc-danger", error: "text-fc-danger" };
  const ACTIONS = [
    ["check", "Check all"],
    ["build", "Build all"],
    ["body", "Bodies"],
  ];
  let logEl = $state<HTMLElement>();

  $effect(() => {
    void log;
    if (logEl) logEl.scrollTop = logEl.scrollHeight;
  });
</script>

<div class="flex flex-col gap-2">
  <div class="inline-flex gap-1 self-start rounded-full bg-fc-component p-1">
    {#each ACTIONS as [action, label] (action)}
      <Button variant="ghost" size="sm" disabled={busy} onclick={() => launch(action)}>{label}</Button>
    {/each}
  </div>
  {#if runs.error}
    <div class="rounded-2xl bg-fc-danger/10 px-4 py-3 text-fc-sm text-fc-danger" role="alert">{runs.error}</div>
  {/if}
  {#each [...runs.list].reverse() as run (run.id)}
    <button
      type="button"
      class="flex items-center justify-between gap-3 rounded-2xl px-4 py-2 text-left text-fc-sm hover:bg-fc-component focus-visible:outline-2 focus-visible:outline-fc-ring {run.id === runs.active
        ? 'bg-fc-component'
        : ''}"
      aria-pressed={run.id === runs.active}
      onclick={() => {
        runs.active = run.id;
        runs.error = "";
      }}
    >
      <span class="truncate">{run.label}</span>
      <span class="shrink-0 text-fc-xs {TONE[run.status]}">{STATUS[run.status] || run.status}</span>
    </button>
  {:else}
    <EmptyState bare title="No runs yet" description="Press play on an item in the project." />
  {/each}
  <pre bind:this={logEl} class="max-h-80 overflow-auto rounded-2xl bg-fc-component p-4 font-fc-mono text-fc-xs whitespace-pre-wrap select-text" aria-label="Run log">{log}</pre>
</div>
