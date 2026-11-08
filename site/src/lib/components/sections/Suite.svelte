<script lang="ts">
  import { Button } from "@facile/muse";
  import type { Content } from "$lib/content/types";
  import { scrollReveal } from "$lib/motion";

  let { content }: { content: Content } = $props();
</script>

<section
  id="suite"
  aria-labelledby="suite-title"
  use:scrollReveal={{ stagger: 0.06 }}
  class="flex flex-col items-start gap-5 rounded-fc-lg bg-fc-surface p-6 sm:p-10"
>
  <div data-reveal-item class="flex flex-col gap-2">
    <h2 id="suite-title" class="text-fc-xl font-semibold tracking-tight text-fc-fg sm:text-fc-2xl">
      {content.suite.title}
    </h2>
    <p class="text-fc-md text-fc-fg-muted">{content.suite.text}</p>
  </div>
  <div data-reveal-item class="flex flex-wrap gap-2">
    {#each content.suite.links as link (link.href)}
      <Button
        variant="secondary"
        size="md"
        href={link.href}
        target="_blank"
        rel="noreferrer">{link.label}</Button
      >
    {/each}
  </div>
</section>
