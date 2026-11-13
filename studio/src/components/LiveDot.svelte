<script lang="ts">
  import { Spinner, StatusDot, Tooltip } from "@facile/muse";
  import { events } from "../lib/events.svelte";
  import { project } from "../lib/project.svelte";

  const label = $derived(
    project.compiling
      ? "Compiling circuit..."
      : events.connected
        ? "Live reload connected"
        : "Reconnecting to the dev server"
  );
</script>

<Tooltip text={label} describe={false} class="contents">
{#if project.compiling}
  <span
    class="inline-flex h-9 shrink-0 items-center gap-1.5 px-2"
    role="status"
    aria-label={label}
  >
    <StatusDot tone="warning" pulse />
    <Spinner size="sm" class="size-3.5 text-fc-warning" />
  </span>
{:else}
  <span
    class="inline-flex size-9 shrink-0 items-center justify-center"
    role="status"
    aria-label={label}
  >
    <StatusDot tone={events.connected ? "success" : "danger"} pulse={!events.connected} />
  </span>
{/if}
</Tooltip>
