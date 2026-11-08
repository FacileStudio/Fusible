<script lang="ts">
  import type { Content } from "$lib/content/types";
  import { scrollReveal } from "$lib/motion";
  import CodeBlock from "../CodeBlock.svelte";
  import AgentDemo from "../demos/AgentDemo.svelte";

  let { content }: { content: Content } = $props();
  const agents = $derived(content.agents);
</script>

<section
  id="agents"
  aria-labelledby="agents-title"
  use:scrollReveal={{ stagger: 0.08 }}
  class="grid grid-cols-1 gap-8 rounded-fc-lg bg-fc-component p-6 sm:p-10 lg:grid-cols-2 lg:items-center"
>
  <div data-reveal-item class="flex flex-col gap-4">
    <h2 id="agents-title" class="text-fc-xl font-semibold tracking-tight text-fc-fg sm:text-fc-2xl">
      {agents.title}
    </h2>
    <p class="text-fc-md text-fc-fg-muted">{agents.text}</p>
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
