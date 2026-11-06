<script lang="ts">
  import { Button, EmptyState } from "@facile/muse";

  type Props = { onNew: () => void; onOpen: () => void };
  let { onNew, onOpen }: Props = $props();
</script>

<EmptyState
  bare
  icon="solar:cpu-bolt-bold-duotone"
  title="No projects yet"
  description="Create a project or open a folder that has a fusible.yml."
>
  <div class="flex gap-2">
    <Button icon="solar:add-circle-linear" onclick={onNew}>New project</Button>
    <Button variant="ghost" icon="solar:folder-open-linear" onclick={onOpen}>Open folder</Button>
  </div>
</EmptyState>
