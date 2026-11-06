<script lang="ts">
  import { Icon } from "@facile/muse";
  import type { Content } from "$lib/content/types";
  import { scrollReveal } from "$lib/motion";

  let { content }: { content: Content } = $props();
</script>

<section id="features" aria-labelledby="features-title" use:scrollReveal={{ stagger: 0.06 }} class="flex flex-col gap-6">
  <h2 id="features-title" class="text-fc-xl font-semibold tracking-tight text-fc-fg sm:text-fc-2xl">
    {content.pillars.title}
  </h2>
  <ul class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
    {#each content.pillars.items as item (item.title)}
      <li data-reveal-item class="flex flex-col gap-3 rounded-3xl bg-fc-surface p-6">
        <span class="inline-flex size-12 items-center justify-center rounded-full bg-fc-component text-fc-fg">
          <Icon icon={item.icon} size={26} />
        </span>
        <h3 class="text-fc-lg font-semibold text-fc-fg">{item.title}</h3>
        <p class="text-fc-sm text-fc-fg-muted">{item.text}</p>
      </li>
    {/each}
  </ul>
</section>
