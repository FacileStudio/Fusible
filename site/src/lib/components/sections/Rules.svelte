<script lang="ts">
  import { Icon } from "@facile/muse";
  import type { Content } from "$lib/content/types";
  import { scrollReveal } from "$lib/motion";
  import Pill from "../Pill.svelte";

  let { content }: { content: Content } = $props();
</script>

<section id="rules" aria-labelledby="rules-title" use:scrollReveal={{ stagger: 0.04 }} class="flex flex-col gap-6">
  <div class="flex max-w-2xl flex-col gap-2">
    <h2 id="rules-title" class="text-fc-xl font-semibold tracking-tight text-fc-fg sm:text-fc-2xl">
      {content.rules.title}
    </h2>
    <p class="text-fc-md text-fc-fg-muted">{content.rules.intro}</p>
  </div>
  <ul class="flex flex-col gap-2 rounded-3xl bg-fc-component p-3 sm:p-4">
    {#each content.rules.items as rule (rule.id)}
      <li
        data-reveal-item
        class="grid grid-cols-1 gap-2 rounded-2xl bg-fc-surface px-5 py-4 md:grid-cols-[14rem_11rem_1fr] md:items-center md:gap-4"
      >
        <code class="font-mono text-fc-sm font-medium text-fc-fg">{rule.id}</code>
        <div>
          {#if rule.standard}
            <Pill href={rule.standard.href}>{rule.standard.label}</Pill>
          {/if}
        </div>
        <p class="text-fc-sm text-fc-fg-muted">{rule.summary}</p>
      </li>
    {/each}
  </ul>
  <p class="flex items-center gap-2 text-fc-sm text-fc-fg-muted">
    <Icon icon="solar:shield-check-linear" size={18} />
    <span>{content.rules.disclaimer}</span>
  </p>
</section>
