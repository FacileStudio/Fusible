<script lang="ts">
  import type { Content } from "$lib/content/types";
  import { scrollReveal } from "$lib/motion";
  import CheckTile from "../demos/CheckTile.svelte";
  import CostTile from "../demos/CostTile.svelte";
  import FirmwareTile from "../demos/FirmwareTile.svelte";

  let { content }: { content: Content } = $props();
</script>

<section id="features" aria-labelledby="features-title" use:scrollReveal={{ stagger: 0.08 }} class="flex flex-col gap-6">
  <div data-reveal-item class="flex flex-col gap-2">
    <h2 id="features-title" class="text-fc-xl font-semibold tracking-tight text-fc-fg sm:text-fc-2xl">{content.demos.title}</h2>
    <p class="text-fc-md text-fc-fg-muted">{content.demos.intro}</p>
  </div>
  <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
    <CheckTile copy={content.demos.check} />
    <FirmwareTile copy={content.demos.firmware} />
    <CostTile copy={content.demos.cost} />
  </div>
</section>
