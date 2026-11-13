<script lang="ts">
  import Landing from "$lib/components/Landing.svelte";
  import { en } from "$lib/content/en";

  let { data } = $props();
</script>

<Landing content={en} release={data.release} />
