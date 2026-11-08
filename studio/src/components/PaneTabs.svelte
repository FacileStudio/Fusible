<script lang="ts">
  import { Icon, Tooltip } from "@facile/muse";
  import { studio } from "../lib/studio.svelte";
  import type { PaneName } from "../lib/types";

  type Pane = { id: string; label: string; icon: string };
  let { items }: { items: Pane[] } = $props();

  const hasIssues = $derived(studio.diagnostics.length > 0);

  function move(event: KeyboardEvent, index: number) {
    const step = { ArrowRight: 1, ArrowLeft: -1 }[event.key];
    if (!step) return;
    event.preventDefault();
    const next = (index + step + items.length) % items.length;
    studio.pane = items[next].id as PaneName;
    ((event.currentTarget as HTMLElement).closest("[role=tablist]")?.querySelectorAll("button")[next] as HTMLElement)?.focus();
  }
</script>

<div role="tablist" aria-label="Panels" class="grid shrink-0 auto-cols-fr grid-flow-col gap-1 rounded-fc-pill bg-fc-surface p-1">
  {#each items as item, index (item.id)}
    {@const active = studio.pane === item.id}
    <Tooltip text={item.label}>
      <button
        type="button"
        role="tab"
        aria-selected={active}
        aria-label={item.label}
        aria-controls="pane-panel"
        tabindex={active ? 0 : -1}
        class="relative inline-flex h-9 w-full items-center justify-center rounded-fc-pill transition-colors pointer-coarse:h-11 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-fc-ring {active
          ? 'bg-fc-accent text-fc-accent-fg'
          : 'text-fc-fg-muted hover:text-fc-fg'}"
        onclick={() => (studio.pane = item.id as PaneName)}
        onkeydown={(event) => move(event, index)}
      >
        <Icon icon={item.icon} size={18} />
        {#if item.id === "checks" && hasIssues}
          <span class="absolute right-2 top-1.5 size-1.5 rounded-fc-full bg-fc-danger" aria-hidden="true"></span>
        {/if}
      </button>
    </Tooltip>
  {/each}
</div>
