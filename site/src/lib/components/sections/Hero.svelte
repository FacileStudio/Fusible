<script lang="ts">
  import { onMount } from "svelte";
  import { gsap } from "gsap";
  import { Button, icons, prefersReducedMotion } from "@facile/muse";
  import type { Content } from "$lib/content/types";
  import CopyButton from "../CopyButton.svelte";
  import { scrollReveal } from "$lib/motion";

  let { content }: { content: Content } = $props();

  let frame: HTMLDivElement | null = $state(null);
  const command = "cargo install fusible";

  onMount(() => {
    if (!frame || prefersReducedMotion()) return;
    const tween = gsap.fromTo(
      frame,
      { opacity: 0, y: 48, scale: 0.97 },
      { opacity: 1, y: 0, scale: 1, duration: 0.9, delay: 0.35, ease: "power3.out", clearProps: "transform,opacity" },
    );
    return () => tween.kill();
  });
</script>

<section use:scrollReveal={{ delay: 0.1, stagger: 0.08 }} class="flex flex-col gap-6 pt-4 md:pt-10">
  <div data-reveal-item class="flex max-w-3xl flex-col gap-4">
    <h1 class="text-fc-2xl font-semibold tracking-tight text-fc-fg sm:text-fc-3xl">{content.hero.title}</h1>
    <p class="text-fc-md text-fc-fg-muted sm:text-fc-lg">{content.hero.sub}</p>
  </div>

  <div data-reveal-item class="flex flex-wrap items-center gap-3">
    <Button variant="primary" size="lg" href="{content.prefix}/download" icon={icons.download}>
      {content.hero.download}
    </Button>
    <div class="inline-flex h-11 items-center gap-2 rounded-full bg-fc-surface pl-5 pr-1.5">
      <code class="font-mono text-fc-sm text-fc-fg">{command}</code>
      <CopyButton text={command} {content} label={content.hero.copyLabel} />
    </div>
  </div>

  <div bind:this={frame} class="rounded-3xl bg-fc-surface p-2">
    <img
      src="/screenshots/studio-pcb.png"
      alt={content.hero.shotAlt}
      width="1600"
      height="1000"
      fetchpriority="high"
      class="h-auto w-full rounded-[1.25rem]"
    />
  </div>
</section>
