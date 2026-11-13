<script lang="ts">
  import type { Content } from "$lib/content/types";
  import type { Release } from "$lib/release";
  import Downloads from "../Downloads.svelte";

  let { content, release }: { content: Content; release: Release } = $props();
</script>

<section id="download" aria-labelledby="download-title" class="flex scroll-mt-24 flex-col gap-6">
  <h2 id="download-title" class="text-fc-xl font-bold tracking-tight text-fc-fg sm:text-fc-2xl">
    {content.download.heading}
  </h2>
  <Downloads {content} {release} />
</section>
