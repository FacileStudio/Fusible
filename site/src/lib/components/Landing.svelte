<script lang="ts">
  import type { Content } from "$lib/content/types";
  import Seo from "./Seo.svelte";
  import SiteShell from "./SiteShell.svelte";
  import Agents from "./sections/Agents.svelte";
  import FuseLoop from "./sections/FuseLoop.svelte";
  import Hero from "./sections/Hero.svelte";
  import Pillars from "./sections/Pillars.svelte";
  import Rules from "./sections/Rules.svelte";
  import ShotSection from "./sections/ShotSection.svelte";
  import Suite from "./sections/Suite.svelte";

  let { content }: { content: Content } = $props();
</script>

<Seo {content} meta={content.meta.home} page="" jsonLd />
<SiteShell {content}>
  <Hero {content} />
  <FuseLoop {content} />
  <Pillars {content} />
  <ShotSection id="board3d" data={content.board3d} {content} src="/screenshots/studio-3d.png" width={1600} height={1000} />
  <ShotSection id="checks" data={content.checks} {content} src="/screenshots/studio-checks.png" width={1600} height={1000} reverse />
  <Rules {content} />
  <Agents {content} />
  <ShotSection id="bodies" data={content.bodies} {content} src="/screenshots/studio-bodies.png" width={1600} height={1000} />
  <ShotSection id="mobile" data={content.mobile} {content} src="/screenshots/mobile.png" width={600} height={1300} phone />
  <ShotSection id="desktop" data={content.desktop} {content} src="/screenshots/desktop.png" width={1600} height={1000} reverse />
  <Suite {content} />
</SiteShell>
