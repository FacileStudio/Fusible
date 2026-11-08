<script lang="ts">
  import { nav } from "../lib/nav.svelte";
  import { updater } from "../lib/updater.svelte";

  const visible = $derived(updater.state === "available" || updater.state === "ready");
  const label = $derived(updater.state === "ready" ? "Restart to update" : `Update ${updater.version ?? ""}`.trim());
</script>

{#if visible}
  <button
    type="button"
    class="rounded-fc-pill bg-fc-accent px-3 py-1 text-fc-xs font-medium text-fc-accent-fg hover:opacity-90 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-fc-ring"
    onclick={() => nav.go("settings", "updates")}
  >
    {label}
  </button>
{/if}
