<script lang="ts">
  import Landing from "$lib/components/Landing.svelte";
  import { de } from "$lib/content/de";

  let { data } = $props();
</script>

<Landing content={de} release={data.release} />
