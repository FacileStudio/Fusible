<script lang="ts">
  import { onMount, tick } from "svelte";
  import { Icon, prefersReducedMotion } from "@facile/muse";
  import type { Content } from "$lib/content/types";
  import { pinHorizontal } from "$lib/motion";

  let { content }: { content: Content } = $props();

  let section: HTMLElement | null = $state(null);
  let track: HTMLElement | null = $state(null);
  let pinned = $state(false);

  onMount(() => {
    if (!section || !track || prefersReducedMotion()) return;
    let stop = () => {};
    let alive = true;
    pinned = true;
    tick().then(() => pinHorizontal(section!, track!)).then((cleanup) => {
      if (alive) stop = cleanup;
      else cleanup();
    });
    return () => {
      alive = false;
      stop();
    };
  });
</script>

<section
  id="loop"
  bind:this={section}
  aria-labelledby="loop-title"
  class="flex flex-col gap-6 {pinned ? 'lg:overflow-hidden' : ''}"
>
  <h2 id="loop-title" class="text-fc-xl font-semibold tracking-tight text-fc-fg sm:text-fc-2xl">
    {content.loop.title}
  </h2>
  <ol
    bind:this={track}
    class="grid grid-cols-1 gap-4 {pinned ? 'lg:flex lg:w-max lg:flex-row lg:gap-6' : 'lg:grid-cols-2'}"
  >
    {#each content.loop.steps as step, i (step.title)}
      <li class="flex flex-col gap-4 rounded-3xl bg-fc-surface p-6 lg:p-8 {pinned ? 'lg:min-h-72 lg:w-[30rem]' : ''}">
        <div class="flex items-center gap-3">
          <span class="inline-flex size-12 items-center justify-center rounded-full bg-fc-component text-fc-fg">
            <Icon icon={step.icon} size={24} />
          </span>
          <span class="font-mono text-fc-sm text-fc-fg-muted">0{i + 1}</span>
        </div>
        <h3 class="text-fc-xl font-semibold text-fc-fg">{step.title}</h3>
        <p class="text-fc-md text-fc-fg-muted">{step.text}</p>
      </li>
    {/each}
  </ol>
</section>
