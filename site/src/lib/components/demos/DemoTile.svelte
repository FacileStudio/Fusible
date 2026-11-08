<script lang="ts">
  import type { Snippet } from "svelte";

  let { title, text, children }: { title: string; text: string; children: Snippet } = $props();
</script>

<article data-reveal-item class="flex flex-col overflow-hidden rounded-fc-lg bg-fc-component">
  <div class="relative">{@render children()}</div>
  <div class="flex flex-col gap-1.5 px-6 pb-6 pt-2">
    <h3 class="text-fc-lg font-semibold text-fc-fg">{title}</h3>
    <p class="text-fc-sm text-fc-fg-muted">{text}</p>
  </div>
</article>
