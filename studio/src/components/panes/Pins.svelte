<script lang="ts">
  import { EmptyState } from "@facile/muse";
  import FirmwareView from "../FirmwareView.svelte";
  import { firmware } from "../../lib/firmware.svelte";
</script>

{#if firmware.name}
  <FirmwareView />
{:else}
  <EmptyState bare title="No firmware selected" description="Select a firmware in the project tree." />
{/if}
