<script lang="ts">
  import { Icon } from "@facile/muse";
  import type { Content } from "$lib/content/types";
  import { scrollReveal } from "$lib/motion";

  let { content }: { content: Content } = $props();
</script>

<section
  id="loop"
  aria-labelledby="loop-title"
  use:scrollReveal={{ stagger: 0.08 }}
  class="flex flex-col gap-6"
>
  <h2 id="loop-title" data-reveal-item class="text-fc-xl font-semibold tracking-tight text-fc-fg sm:text-fc-2xl">
    {content.loop.title}
  </h2>
  <ol class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
    {#each content.loop.steps as step, i (step.title)}
      <li data-reveal-item class="flex flex-col gap-4 rounded-3xl bg-fc-surface p-6">
        <div class="flex items-center gap-3">
          <span class="inline-flex size-12 items-center justify-center rounded-full bg-fc-component text-fc-fg">
            <Icon icon={step.icon} size={24} />
          </span>
          <span class="font-mono text-fc-sm text-fc-fg-muted">0{i + 1}</span>
        </div>
        <h3 class="text-fc-lg font-semibold text-fc-fg">{step.title}</h3>
        <p class="text-fc-md text-fc-fg-muted">{step.text}</p>
      </li>
    {/each}
  </ol>
</section>
