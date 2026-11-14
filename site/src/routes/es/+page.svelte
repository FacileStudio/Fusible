<script lang="ts">
  import Landing from "$lib/components/Landing.svelte";
  import { es } from "$lib/content/es";

  let { data } = $props();
</script>

<Landing content={es} release={data.release} />
