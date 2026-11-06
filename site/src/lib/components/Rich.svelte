<script lang="ts">
  let { text }: { text: string } = $props();

  const parts = $derived(text.split("`"));
</script>

{#each parts as part, i (i)}{#if i % 2 === 1}<code class="rounded-md bg-fc-surface px-1.5 py-0.5 font-mono text-fc-xs text-fc-fg whitespace-nowrap">{part}</code>{:else}{part}{/if}{/each}
