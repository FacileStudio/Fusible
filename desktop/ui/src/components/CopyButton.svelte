<script lang="ts">
  import { Icon, IconButton } from "@facile/muse";
  import { copyText } from "../lib/clipboard";

  let { text, label = "Copy" }: { text: string; label?: string } = $props();
  let done = $state(false);

  async function copy(): Promise<void> {
    done = await copyText(text);
    setTimeout(() => (done = false), 1500);
  }
</script>

<IconButton variant="ghost" aria-label={done ? "Copied" : label} onclick={copy}>
  <Icon icon={done ? "solar:check-circle-linear" : "solar:copy-linear"} />
</IconButton>
