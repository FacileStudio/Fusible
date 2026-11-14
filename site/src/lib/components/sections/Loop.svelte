<script lang="ts">
  import type { Content } from "$lib/content/types";
  import RouteTile from "../demos/RouteTile.svelte";
  import ShortTile from "../demos/ShortTile.svelte";
  import SyncTile from "../demos/SyncTile.svelte";

  let { content }: { content: Content } = $props();
</script>

<section aria-labelledby="loop-title" class="relative">
  <h2 id="loop-title" class="mb-10 text-center font-fc-title text-2xl font-bold tracking-tight text-fc-fg sm:text-3xl">
    {content.demos.loop.title}
  </h2>
  <div class="grid gap-6 md:grid-cols-3">
    <ShortTile copy={content.demos.loop.short} />
    <RouteTile copy={content.demos.loop.route} />
    <SyncTile copy={content.demos.loop.sync} />
  </div>
</section>
