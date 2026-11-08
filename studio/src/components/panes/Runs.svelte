<script lang="ts">
  import { Alert, Button, EmptyState } from "@facile/muse";
  import { launch } from "../../lib/dispatch.svelte";
  import { isBusy, runs } from "../../lib/runs.svelte";

  const busy = $derived(isBusy());
  const log = $derived(runs.list.find((run) => run.id === runs.active)?.lines.join("\n") ?? "");
  const STATUS = { running: "Running", ok: "OK", fuse_blown: "Fuse blown", error: "Failed" };
  const TONE = { running: "text-fc-info", ok: "text-fc-success", fuse_blown: "text-fc-danger", error: "text-fc-danger" };
  const ACTIONS = [
    ["check", "Check all"],
    ["build", "Build all"],
    ["body", "Build bodies"],
  ];
  let logEl = $state<HTMLElement>();

  $effect(() => {
    void log;
    if (logEl) logEl.scrollTop = logEl.scrollHeight;
  });
</script>

<div class="flex flex-col gap-2">
  <div class="flex flex-wrap gap-1">
    {#each ACTIONS as [action, label] (action)}
      <Button variant="secondary" size="sm" disabled={busy} onclick={() => launch(action)}>{label}</Button>
    {/each}
  </div>
  {#if runs.error}
    <Alert tone="danger">{runs.error}</Alert>
  {/if}
  {#each [...runs.list].reverse() as run (run.id)}
    <button
      type="button"
      class="flex items-center justify-between gap-3 rounded-fc-lg px-3 py-2 text-left text-fc-sm hover:bg-fc-surface focus-visible:outline-2 focus-visible:outline-fc-ring {run.id === runs.active
        ? 'bg-fc-surface'
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
  {#if log}
    <pre bind:this={logEl} class="max-h-80 overflow-auto rounded-fc-lg bg-fc-surface p-4 font-fc-mono text-fc-xs whitespace-pre-wrap select-text" aria-label="Run log">{log}</pre>
  {/if}
</div>
