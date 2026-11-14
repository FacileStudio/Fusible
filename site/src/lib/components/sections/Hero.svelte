<script lang="ts">
  import { Button, icons } from "@facile/muse";
  import type { Content } from "$lib/content/types";
  import CopyButton from "../CopyButton.svelte";
  import { scrollReveal } from "$lib/motion";
  import HeroDemo from "../demos/HeroDemo.svelte";

  let { content }: { content: Content } = $props();

  const command = "cargo install fusible";
</script>

<section use:scrollReveal={{ delay: 0.1, stagger: 0.08 }} class="flex flex-col gap-6 pt-4 md:pt-10">
  <div data-reveal-item class="flex max-w-3xl flex-col gap-4">
    <h1 class="text-fc-2xl font-bold tracking-tight text-fc-fg sm:text-fc-3xl">{content.hero.title}</h1>
    <p class="text-fc-md text-fc-fg-muted sm:text-fc-lg">{content.hero.sub}</p>
  </div>

  <div data-reveal-item class="flex flex-wrap items-center gap-3">
    <Button variant="primary" size="lg" href="#download" icon={icons.download}>
      {content.hero.download}
    </Button>
    <div class="inline-flex h-11 max-w-full items-center gap-2 rounded-fc-pill bg-fc-surface pl-5 pr-1.5">
      <code class="truncate font-fc-mono text-fc-sm text-fc-fg">{command}</code>
      <CopyButton text={command} {content} label={content.hero.copyLabel} />
    </div>
    <p class="basis-full text-fc-sm text-fc-fg-muted">{content.hero.note}</p>
  </div>

  <div data-reveal-item>
    <HeroDemo copy={content.demos.hero} />
  </div>
</section>
