<script lang="ts">
  import { Button, icons } from "@facile/muse";
  import type { Content } from "$lib/content/types";

  let { content }: { content: Content } = $props();
</script>

<section aria-labelledby="closing-title" class="flex flex-col items-start gap-5 rounded-fc-lg bg-fc-surface p-6 sm:p-10">
  <h2 id="closing-title" class="text-fc-xl font-bold tracking-tight text-fc-fg sm:text-fc-2xl">{content.closing.title}</h2>
  <div class="flex flex-wrap gap-3">
    <Button variant="primary" size="lg" href="#download" icon={icons.download}>{content.hero.download}</Button>
    <Button variant="secondary" size="lg" href="{content.prefix}/docs">{content.nav.docs}</Button>
  </div>
</section>
