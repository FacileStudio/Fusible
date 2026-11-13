<script lang="ts">
  import type { Content } from "$lib/content/types";
  import { scrollReveal } from "$lib/motion";
  import CheckTile from "../demos/CheckTile.svelte";
  import CostTile from "../demos/CostTile.svelte";
  import FirmwareTile from "../demos/FirmwareTile.svelte";

  let { content }: { content: Content } = $props();
</script>

<section id="features" aria-labelledby="features-title" use:scrollReveal class="flex flex-col gap-6">
  <h2 id="features-title" class="text-fc-xl font-bold tracking-tight text-fc-fg sm:text-fc-2xl">{content.demos.title}</h2>
  <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
    <CheckTile copy={content.demos.check} />
    <FirmwareTile copy={content.demos.firmware} />
    <CostTile copy={content.demos.cost} />
  </div>
</section>
