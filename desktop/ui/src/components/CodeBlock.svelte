<script lang="ts">
  import CopyButton from "./CopyButton.svelte";

  let { title, code }: { title?: string; code: string } = $props();
</script>

<figure class="flex flex-col gap-2">
  {#if title}<figcaption class="text-fc-sm font-medium">{title}</figcaption>{/if}
  <div class="flex items-start gap-2 rounded-2xl bg-fc-surface p-3">
    <pre class="min-w-0 flex-1 overflow-x-auto font-mono text-fc-xs leading-relaxed"><code>{code}</code></pre>
    <CopyButton text={code} label="Copy {title ?? 'code'}" />
  </div>
</figure>
