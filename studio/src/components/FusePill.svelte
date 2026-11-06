<script lang="ts">
  import { Icon } from "@facile/muse";
  import { openPane, studio } from "../lib/studio.svelte";

  const count = $derived(studio.diagnostics.length);
</script>

<button
  type="button"
  class="inline-flex h-9 shrink-0 items-center gap-2 rounded-full px-4 text-fc-sm font-medium transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-fc-ring {studio.fuseBlown
    ? 'bg-fc-danger text-fc-danger-fg'
    : 'bg-fc-surface text-fc-fg'}"
  title="Show checks"
  onclick={() => openPane("checks")}
>
  <Icon
    icon={studio.fuseBlown ? "solar:danger-triangle-bold-duotone" : "solar:shield-check-bold-duotone"}
    size={18}
  />
  <span class="max-lg:hidden">{studio.fuseBlown ? `Fuse blown · ${count}` : "Fuse intact"}</span>
</button>
