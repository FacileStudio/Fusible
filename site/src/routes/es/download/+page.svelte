<script lang="ts">
  import DownloadPage from "$lib/components/DownloadPage.svelte";
  import { es } from "$lib/content/es";

  let { data } = $props();
</script>

<DownloadPage content={es} release={data.release} />
