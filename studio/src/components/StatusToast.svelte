<script lang="ts">
  import { toast } from "@facile/muse";
  import { studio } from "../lib/studio.svelte";

  $effect(() => {
    if (studio.status) toast.warning(studio.status, { duration: 8000 });
  });
</script>
