<script lang="ts">
  import DownloadPage from "$lib/components/DownloadPage.svelte";
  import { en } from "$lib/content/en";

  let { data } = $props();
</script>

<DownloadPage content={en} release={data.release} />
