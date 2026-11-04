<script lang="ts">
  import { StatusDot } from "@facile/muse";
  import { events } from "../lib/events.svelte";
</script>

<span class="inline-flex size-9 shrink-0 items-center justify-center">
  <StatusDot
    tone={events.connected ? "success" : "danger"}
    label={events.connected ? "Live reload connected" : "Reconnecting to the dev server"}
    pulse={!events.connected}
  />
</span>
