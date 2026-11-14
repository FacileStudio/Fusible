<script lang="ts">
  import type { Content } from "$lib/content/types";
  import type { Release } from "$lib/release";
  import Seo from "./Seo.svelte";
  import SiteShell from "./SiteShell.svelte";
  import Agents from "./sections/Agents.svelte";
  import Board3D from "./sections/Board3D.svelte";
  import Closing from "./sections/Closing.svelte";
  import Demos from "./sections/Demos.svelte";
  import Developers from "./sections/Developers.svelte";
  import Download from "./sections/Download.svelte";
  import Hero from "./sections/Hero.svelte";
  import Loop from "./sections/Loop.svelte";
  import Suite from "./sections/Suite.svelte";

  let { content, release }: { content: Content; release: Release } = $props();
</script>

<Seo {content} meta={content.meta.home} page="" jsonLd />
<SiteShell {content}>
  <Hero {content} />
  <Loop {content} />
  <Board3D {content} />
  <Demos {content} />
  <Agents {content} />
  <Suite {content} />
  <Download {content} {release} />
  <Developers {content} {release} />
  <Closing {content} />
</SiteShell>
