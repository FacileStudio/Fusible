<script lang="ts">
  import { Icon, IconButton } from "@facile/muse";
  import { LABEL, WINDOW_BUTTONS, invoke } from "../lib/desktop";
</script>

<div class="flex shrink-0 items-center gap-1" role="group" aria-label="Window controls">
  {#each WINDOW_BUTTONS as button (button.name)}
    <IconButton variant="ghost" class="size-8" aria-label={button.label} onclick={() => invoke(button.cmd, { label: LABEL })}>
      <Icon icon={button.icon} size={16} />
    </IconButton>
  {/each}
</div>
