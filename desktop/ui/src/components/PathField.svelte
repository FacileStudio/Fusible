<script lang="ts">
  import { Button, Input } from "@facile/muse";
  import { api } from "../lib/api";

  type Props = { value: string; title: string; onPick?: (path: string) => void; label?: string };
  let { value = $bindable(""), title, onPick, label = "Folder path" }: Props = $props();

  async function browse(): Promise<void> {
    const picked = await api.pickFolder(title, value || undefined).catch(() => null);
    if (!picked) return;
    value = picked;
    onPick?.(picked);
  }
</script>

<div class="flex items-center gap-2">
  <Input bind:value aria-label={label} spellcheck={false} class="border-0 bg-fc-surface" onchange={() => onPick?.(value.trim())} />
  <Button variant="ghost" icon="solar:folder-open-linear" onclick={browse}>Browse</Button>
</div>
