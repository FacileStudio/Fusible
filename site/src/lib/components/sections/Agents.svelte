<script lang="ts">
  import { Blob, gaze } from "@facile/blob";
  import { Icon } from "@facile/muse";
  import type { Content } from "$lib/content/types";
  import { BULLE_URL } from "$lib/content/shared";
  import { scrollReveal } from "$lib/motion";
  import AgentDemo from "../demos/AgentDemo.svelte";

  let { content }: { content: Content } = $props();
  const eyes = gaze({ travel: 5, target: "pointer" });
</script>

<section
  id="agents"
  aria-labelledby="agents-title"
  use:scrollReveal={{ stagger: 0.08 }}
  class="grid grid-cols-1 gap-8 rounded-fc-lg bg-fc-component p-6 sm:p-10 lg:grid-cols-2 lg:items-center"
>
  <div data-reveal-item class="flex flex-col items-start gap-4">
    <a
      href={BULLE_URL}
      target="_blank"
      rel="noreferrer"
      class="group inline-flex items-center gap-3 rounded-fc-pill bg-fc-surface px-3 py-1.5 text-fc-sm text-fc-fg transition-colors hover:bg-fc-field"
    >
      <Blob {@attach eyes} name="Bulle" size={26} animate="always" class="shrink-0" />
      <span class="font-bold">Bulle</span>
      <Icon
        icon="solar:arrow-right-up-linear"
        size={14}
        class="text-fc-fg-muted transition-transform group-hover:translate-x-0.5 group-hover:-translate-y-0.5 group-hover:text-fc-fg"
      />
    </a>
    <h2 id="agents-title" class="text-fc-xl font-bold tracking-tight text-fc-fg sm:text-fc-2xl">
      {content.agents.title}
    </h2>
    <p class="text-fc-md text-fc-fg-muted">{content.agents.text}</p>
  </div>
  <div data-reveal-item class="min-w-0">
    <AgentDemo copy={content.demos.agent} />
  </div>
</section>
