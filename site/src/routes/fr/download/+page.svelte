<script lang="ts">
  import DownloadPage from "$lib/components/DownloadPage.svelte";
  import { fr } from "$lib/content/fr";

  let { data } = $props();
</script>

<DownloadPage content={fr} release={data.release} />
