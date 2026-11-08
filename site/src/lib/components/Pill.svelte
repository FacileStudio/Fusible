<script lang="ts">
  import type { Snippet } from "svelte";

  let { href, children }: { href: string; children: Snippet } = $props();
</script>

<a
  {href}
  target="_blank"
  rel="noreferrer"
  class="inline-flex w-fit items-center rounded-fc-pill bg-fc-component px-3 py-1 text-fc-xs font-medium text-fc-fg-muted
    transition-colors hover:text-fc-fg focus-visible:outline-2 focus-visible:outline-offset-2
    focus-visible:outline-fc-ring">{@render children()}</a
>
