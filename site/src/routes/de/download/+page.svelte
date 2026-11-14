<script lang="ts">
  import DownloadPage from "$lib/components/DownloadPage.svelte";
  import { de } from "$lib/content/de";

  let { data } = $props();
</script>

<DownloadPage content={de} release={data.release} />
