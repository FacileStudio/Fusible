<script lang="ts">
  import { StatusDot } from "@facile/muse";
  import { events } from "../lib/events.svelte";

  const label = $derived(events.connected ? "Live reload connected" : "Reconnecting to the dev server");
</script>

<span class="inline-flex size-9 shrink-0 items-center justify-center" title={label} role="status" aria-label={label}>
  <StatusDot tone={events.connected ? "success" : "danger"} pulse={!events.connected} />
</span>
