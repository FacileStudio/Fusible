<script lang="ts">
  import type { Content } from "$lib/content/types";
  import CopyButton from "./CopyButton.svelte";

  let { code, content, label }: { code: string; content: Content; label?: string } = $props();
</script>

<div class="relative rounded-fc-md bg-fc-surface">
  {#if label}
    <p class="px-4 pt-3 text-fc-xs text-fc-fg-muted">{label}</p>
  {/if}
  <div
    class="overflow-x-auto p-4 pr-14"
    tabindex="0"
    role="textbox"
    aria-readonly="true"
    aria-multiline="true"
    aria-label={label ?? content.ui.code}
  >
    <pre class="font-fc-mono text-fc-xs leading-relaxed text-fc-fg"><code>{code}</code></pre>
  </div>
  <div class="absolute right-2 top-2"><CopyButton text={code} {content} /></div>
</div>
