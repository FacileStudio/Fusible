<script lang="ts">
  import { Icon, Tooltip, icons } from "@facile/muse";
  import type { Content } from "$lib/content/types";

  let { text, content, label }: { text: string; content: Content; label?: string } = $props();

  let copied = $state(false);

  async function copy() {
    try {
      await navigator.clipboard.writeText(text);
      copied = true;
      setTimeout(() => (copied = false), 1800);
    } catch {}
  }
</script>

<Tooltip text={label ?? content.ui.copy} describe={false} class="contents">
  <button
    type="button"
    onclick={copy}
    aria-label={label ?? content.ui.copy}
    class="inline-flex size-8 shrink-0 items-center justify-center rounded-fc-pill bg-fc-component text-fc-fg-muted
      transition-colors hover:text-fc-fg focus-visible:outline-2 focus-visible:outline-offset-2
      focus-visible:outline-fc-ring"
  >
    <Icon icon={copied ? icons.check : icons.copy} size={16} />
    <span class="sr-only" role="status">{copied ? content.ui.copied : ""}</span>
  </button>
</Tooltip>
