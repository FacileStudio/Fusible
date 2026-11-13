<script lang="ts">
  import Landing from "$lib/components/Landing.svelte";
  import { fr } from "$lib/content/fr";

  let { data } = $props();
</script>

<Landing content={fr} release={data.release} />
