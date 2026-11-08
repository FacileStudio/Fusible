<script lang="ts">
  import { Blob, gaze } from "@facile/blob";
  import { Icon } from "@facile/muse";
  import type { Content } from "$lib/content/types";
  import { SUITE_LINKS } from "$lib/content/shared";
  import { scrollReveal } from "$lib/motion";
  import CodeBlock from "../CodeBlock.svelte";
  import AgentDemo from "../demos/AgentDemo.svelte";

  let { content }: { content: Content } = $props();
  const agents = $derived(content.agents);
  const eyes = gaze({ travel: 5, target: "pointer" });
</script>

<section
  id="agents"
  aria-labelledby="agents-title"
  use:scrollReveal={{ stagger: 0.08 }}
  class="grid grid-cols-1 gap-8 rounded-fc-lg bg-fc-component p-6 sm:p-10 lg:grid-cols-2 lg:items-center"
>
  <div data-reveal-item class="flex flex-col gap-4">
    <a
      href={SUITE_LINKS.bulle}
      target="_blank"
      rel="noreferrer"
      class="group inline-flex w-fit items-center gap-3 rounded-full bg-fc-surface px-3 py-1.5 text-fc-sm text-fc-fg transition-colors hover:bg-fc-field"
      aria-label="Bulle"
    >
      <Blob {@attach eyes} name="Bulle" size={26} animate="always" class="shrink-0" />
      <span class="font-medium">Bulle</span>
      <span class="text-fc-xs text-fc-fg-muted">bulle.facile.studio</span>
      <Icon
        icon="solar:arrow-right-up-linear"
        size={14}
        class="text-fc-fg-muted transition-transform group-hover:translate-x-0.5 group-hover:-translate-y-0.5 group-hover:text-fc-fg"
      />
    </a>
    <h2 id="agents-title" class="text-fc-xl font-semibold tracking-tight text-fc-fg sm:text-fc-2xl">
      {agents.title}
    </h2>
    <p class="text-fc-md text-fc-fg-muted">
      {#each agents.text.split("Bulle") as part, i}
        {part}{#if i === 0}<a
          href={SUITE_LINKS.bulle}
          target="_blank"
          rel="noreferrer"
          class="font-medium text-fc-fg underline decoration-fc-border hover:decoration-fc-fg"
          >Bulle</a
        >{/if}
      {/each}
    </p>
    <p class="text-fc-sm text-fc-fg-muted">
      {agents.evidence}
      {#each agents.sources as source, i (source.href)}
        <a
          href={source.href}
          target="_blank"
          rel="noreferrer"
          class="text-fc-xs underline hover:text-fc-fg">{source.label}</a
        >{i < agents.sources.length - 1 ? ", " : "."}
      {/each}
    </p>
    <CodeBlock code={agents.snippet} {content} label={agents.snippetLabel} />
  </div>
  <div data-reveal-item class="min-w-0">
    <AgentDemo copy={content.demos.agent} />
  </div>
</section>
