<script lang="ts">
  import { onMount } from "svelte";
  import { Toaster, provideLocale } from "@facile/muse";
  import TopBar from "./components/TopBar.svelte";
  import Viewport from "./components/Viewport.svelte";
  import Sidebar from "./components/Sidebar.svelte";
  import ProjectDrawer from "./components/ProjectDrawer.svelte";
  import PairDialog from "./components/PairDialog.svelte";
  import StatusToast from "./components/StatusToast.svelte";
  import HoverTooltip from "./components/HoverTooltip.svelte";
  import { registerStudioIcons } from "./lib/icons";
  import { connectEvents } from "./lib/events.svelte";
  import { handleMessage } from "./lib/dispatch.svelte";
  import { initProject, project } from "./lib/project.svelte";
  import { loadKicad } from "./lib/kicad.svelte";
  import { startGitPolling } from "./lib/git.svelte";
  import { handleKey } from "./lib/keys";
  import { closeOverlay, toggleSidebar, toggleTree, watchNarrow } from "./lib/layout.svelte";
  import { installDesktopKeys } from "./lib/desktop";
  import { setView, studio } from "./lib/studio.svelte";

  registerStudioIcons();
  provideLocale("en");

  const actions = {
    setView,
    toggleSidebar,
    toggleTree,
    closeOverlay,
    hasAssembly: () => studio.hasAssembly,
    hasProject: () => Boolean(project.data),
  };

  onMount(() => {
    const stops = [watchNarrow(), startGitPolling(), installDesktopKeys()];
    void loadKicad();
    void initProject().finally(() => stops.push(connectEvents(handleMessage)));
    return () => stops.forEach((stop) => stop());
  });
</script>

<svelte:window onkeydown={(event) => handleKey(event, actions)} />

<div class="grid h-dvh grid-rows-[56px_1fr] bg-fc-page text-fc-fg">
  <TopBar />
  <main class="flex min-h-0 gap-2 p-2 pt-0">
    <Viewport />
    <Sidebar />
  </main>
</div>

<ProjectDrawer />
<PairDialog />
<HoverTooltip />
<StatusToast />
<Toaster />
