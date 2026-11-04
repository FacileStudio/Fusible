<script lang="ts">
  import { Icon, IconButton, Select } from "@facile/muse";
  import FusePill from "./FusePill.svelte";
  import KicadPanel from "./KicadPanel.svelte";
  import LiveDot from "./LiveDot.svelte";
  import ViewTabs from "./ViewTabs.svelte";
  import WindowControls from "./WindowControls.svelte";
  import { MAC_PILL, isMac, openProjects } from "../lib/desktop";
  import { setTheme, toggleSidebar, toggleTree } from "../lib/layout.svelte";
  import { project, selectItem } from "../lib/project.svelte";
  import { studio } from "../lib/studio.svelte";

  const mac = isMac();
  const boards = $derived(project.data?.boards ?? []);
</script>

<header
  class="flex min-w-0 items-center gap-2 px-2"
  style:padding-left={studio.desktop && mac ? `${MAC_PILL}px` : undefined}
  data-tauri-drag-region
>
  {#if studio.desktop && !mac}
    <WindowControls />
  {/if}
  {#if studio.desktop}
    <button
      type="button"
      id="btn-projects"
      class="inline-flex h-9 shrink-0 items-center gap-2 rounded-full bg-fc-surface px-4 text-fc-sm font-medium focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-fc-ring"
      title="Back to projects"
      onclick={openProjects}
    >
      <Icon icon="solar:alt-arrow-left-linear" size={16} />
      <span class="max-md:hidden">Projects</span>
    </button>
  {/if}
  <IconButton variant="ghost" aria-label="Show project" title="Show project ([)" onclick={toggleTree}>
    <Icon icon="solar:folder-linear" />
  </IconButton>
  <div class="flex shrink-0 items-center gap-2">
    <Icon icon="solar:cpu-bolt-bold-duotone" size={20} class="text-fc-success" />
    <span class="max-w-40 truncate text-fc-sm font-medium max-md:hidden">{project.data?.name || "Fusible"}</span>
  </div>
  {#if boards.length > 1}
    <Select
      value={project.board ?? ""}
      aria-label="Board"
      class="h-9 w-auto rounded-fc-pill border-0 bg-fc-surface text-fc-sm"
      onchange={(event) => selectItem("board", event.currentTarget.value)}
    >
      {#each boards as board (board.name)}
        <option value={board.name}>{board.name}</option>
      {/each}
    </Select>
  {:else if studio.boardName}
    <span class="shrink-0 truncate text-fc-sm text-fc-fg-muted">{studio.boardName}</span>
  {/if}
  <div class="flex min-w-0 flex-1 justify-center overflow-x-auto" data-tauri-drag-region>
    <ViewTabs />
  </div>
  <FusePill />
  <KicadPanel />
  <LiveDot />
  <IconButton variant="ghost" aria-label="Pair a phone" title="Pair a phone" onclick={() => (studio.pairOpen = true)}>
    <Icon icon="solar:qr-code-linear" />
  </IconButton>
  <IconButton
    variant="ghost"
    aria-label={studio.theme === "dark" ? "Switch to light theme" : "Switch to dark theme"}
    onclick={() => setTheme(studio.theme === "dark" ? "light" : "dark")}
  >
    <Icon icon={studio.theme === "dark" ? "solar:sun-linear" : "solar:moon-linear"} />
  </IconButton>
  <IconButton variant="ghost" aria-label="Toggle panel" title="Toggle panel (\)" onclick={toggleSidebar}>
    <Icon icon="solar:sidebar-code-linear" />
  </IconButton>
</header>
