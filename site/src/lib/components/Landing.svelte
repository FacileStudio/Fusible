<script lang="ts">
  import type { Content } from "$lib/content/types";
  import type { Release } from "$lib/release";
  import Seo from "./Seo.svelte";
  import SiteShell from "./SiteShell.svelte";
  import Agents from "./sections/Agents.svelte";
  import Closing from "./sections/Closing.svelte";
  import Demos from "./sections/Demos.svelte";
  import Download from "./sections/Download.svelte";
  import Hero from "./sections/Hero.svelte";

  let { content, release }: { content: Content; release: Release } = $props();
</script>

<Seo {content} meta={content.meta.home} page="" jsonLd />
<SiteShell {content}>
  <Hero {content} />
  <Demos {content} />
  <Agents {content} />
  <Download {content} {release} />
  <Closing {content} />
</SiteShell>
