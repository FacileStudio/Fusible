<script lang="ts">
  import { Icon, IconButton } from "@facile/muse";
  import { api } from "../lib/api";
  import { isMac } from "../lib/platform";

  const mac = isMac();
  const controls = [
    { label: "Minimize", icon: "solar:minimize-square-linear", cmd: "minimize" },
    { label: "Maximize", icon: "solar:maximize-square-linear", cmd: "toggle_maximize" },
    { label: "Close", icon: "solar:close-square-linear", cmd: "close" },
  ];
</script>

<header data-tauri-drag-region class="flex h-11 shrink-0 select-none items-center gap-2 px-3 {mac ? 'pl-24' : ''}">
  <span class="pointer-events-none flex items-center gap-2">
    <Icon icon="solar:cpu-bolt-bold-duotone" size={20} />
    <span class="text-fc-sm font-semibold">Fusible</span>
  </span>
  <span data-tauri-drag-region class="flex-1 self-stretch"></span>
  {#if !mac}
    <div role="group" aria-label="Window controls" class="flex items-center gap-1">
      {#each controls as control (control.cmd)}
        <IconButton variant="ghost" class="size-8" aria-label={control.label} onclick={() => api.windowAction(control.cmd)}>
          <Icon icon={control.icon} size={16} />
        </IconButton>
      {/each}
    </div>
  {/if}
</header>
