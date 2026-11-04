<script lang="ts">
  import { Drawer, Icon, IconButton } from "@facile/muse";
  import SyncView from "./SyncView.svelte";
  import { launch } from "../lib/dispatch.svelte";
  import { GROUPS, itemsOf, subLabel } from "../lib/projectModel";
  import { healthOf, project, selectBom, selectItem } from "../lib/project.svelte";
  import { isBusy } from "../lib/runs.svelte";
  import { studio } from "../lib/studio.svelte";

  const groups = $derived(GROUPS.filter((g) => itemsOf(project.data, g.kind).length));

  function open(kind: string, name: string) {
    selectItem(kind, name);
    studio.treeOpen = false;
  }
</script>

<Drawer bind:open={studio.treeOpen} title={project.data?.name || "Project"} showClose>
  <div class="flex flex-col gap-4" id="project-tree">
    {#each groups as group (group.kind)}
      <div class="flex flex-col gap-1">
        <div class="px-3 text-fc-xs text-fc-fg-muted">{group.title}</div>
        {#each itemsOf(project.data, group.kind) as item (item.name)}
          {@const selected = project.item === `${group.kind}:${item.name}`}
          <div class="flex items-center gap-1 rounded-full {selected ? 'bg-fc-surface' : ''}">
            <button
              type="button"
              class="flex min-w-0 flex-1 items-center gap-3 rounded-full px-3 py-2 text-left text-fc-sm hover:bg-fc-surface focus-visible:outline-2 focus-visible:outline-fc-ring"
              aria-current={selected ? "true" : undefined}
              onclick={() => open(group.kind, item.name)}
            >
              <span
                class="size-2 shrink-0 rounded-full {group.kind === 'board' && healthOf(item.name).fuse_blown ? 'bg-fc-danger' : group.kind === 'board' ? 'bg-fc-success' : 'bg-fc-info'}"
              ></span>
              <span class="min-w-0 flex-1 truncate">{item.name}</span>
              <span class="shrink-0 text-fc-xs text-fc-fg-muted">{subLabel(group.kind, item, healthOf(item.name))}</span>
            </button>
            <IconButton
              variant="ghost"
              class="size-9"
              disabled={isBusy()}
              title={group.hint}
              aria-label={`${group.hint}: ${item.name}`}
              onclick={() => launch(group.action, item.name)}
            >
              <Icon icon="solar:play-linear" />
            </IconButton>
          </div>
        {/each}
      </div>
    {/each}
    <div class="flex flex-col gap-1">
      <div class="px-3 text-fc-xs text-fc-fg-muted">Costs</div>
      <button
        type="button"
        class="flex items-center gap-3 rounded-full px-3 py-2 text-left text-fc-sm hover:bg-fc-surface focus-visible:outline-2 focus-visible:outline-fc-ring {project.item === 'bom:' ? 'bg-fc-surface' : ''}"
        onclick={() => (selectBom(), (studio.treeOpen = false))}
      >
        <Icon icon="solar:bill-list-linear" size={16} />
        <span class="flex-1">Parts and costs</span>
        <span class="text-fc-xs text-fc-fg-muted">BOM</span>
      </button>
    </div>
    <SyncView />
  </div>
</Drawer>
