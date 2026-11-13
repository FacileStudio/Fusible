<script lang="ts">
  import type { Content } from "$lib/content/types";
  import type { Release } from "$lib/release";
  import Downloads from "./Downloads.svelte";
  import Seo from "./Seo.svelte";
  import SiteShell from "./SiteShell.svelte";

  let { content, release }: { content: Content; release: Release } = $props();
  const dl = $derived(content.download);
</script>

<Seo {content} meta={content.meta.download} page="/download" />
<SiteShell {content}>
  <section class="flex flex-col gap-6 pt-4 md:pt-10">
    <div class="flex max-w-2xl flex-col gap-3">
      <h1 class="text-fc-2xl font-bold tracking-tight text-fc-fg sm:text-fc-3xl">{dl.title}</h1>
      <p class="text-fc-md text-fc-fg-muted">{dl.intro}</p>
    </div>
    <Downloads {content} {release} headingLevel={2} />
    <p class="max-w-2xl text-fc-xs text-fc-fg-muted">{dl.note}</p>
    <p class="font-fc-mono text-fc-xs text-fc-fg-muted">{dl.actionLine}</p>
  </section>
</SiteShell>
