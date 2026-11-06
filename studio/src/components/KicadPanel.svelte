<script lang="ts">
  import { kicad, kicadLines, kicadSummary } from "../lib/kicad.svelte";

  const summary = $derived(kicadSummary(kicad.boards));
  const lines = $derived(kicadLines(kicad.boards));
  const open = $derived(kicad.open && summary.kind !== "in_sync");
  const TONES = {
    in_sync: "bg-fc-surface text-fc-success",
    differs: "bg-fc-warning/10 text-fc-warning",
    error: "bg-fc-danger/10 text-fc-danger",
  };
</script>

{#if kicad.boards.length}
  <div class="relative shrink-0">
    <button
      type="button"
      class="inline-flex h-9 items-center rounded-full px-4 text-fc-sm font-medium focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-fc-ring {TONES[summary.kind]}"
      title="KiCad board compared with the design"
      aria-expanded={open}
      onclick={() => (kicad.open = !kicad.open)}
    >
      {summary.text}
    </button>
    {#if open}
      <div class="absolute right-0 top-full z-40 mt-2 max-h-[50dvh] w-85 max-w-[calc(100vw-16px)] overflow-y-auto rounded-3xl bg-fc-surface p-4 text-fc-xs select-text">
        <h2 class="mb-2 text-fc-sm font-medium">
          {summary.kind === "error" ? "KiCad could not be compared" : "KiCad differs from the design"}
        </h2>
        <ul class="flex flex-col gap-1 text-fc-fg-muted">
          {#each lines as line}
            <li class="wrap-anywhere">{line}</li>
          {/each}
        </ul>
        <p class="mt-3 text-fc-fg-muted">
          {summary.kind === "error"
            ? "Fusible changed nothing. Fix the file and save it again."
            : "Fusible only copies part positions from KiCad. It changed nothing for these items: update the TSX to match KiCad, or undo the change in KiCad."}
        </p>
      </div>
    {/if}
  </div>
{/if}
