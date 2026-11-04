<script lang="ts">
  import { Spinner } from "@facile/muse";
  import Onboarding from "./components/onboarding/Onboarding.svelte";
  import ProjectList from "./components/ProjectList.svelte";
  import Settings from "./components/Settings.svelte";
  import Sidebar from "./components/Sidebar.svelte";
  import TitleBar from "./components/TitleBar.svelte";
  import { nav } from "./lib/nav.svelte";
  import { projects } from "./lib/projects.svelte";
  import { settings } from "./lib/settings.svelte";
  import { updater } from "./lib/updater.svelte";

  let booting = $state(true);

  async function boot(): Promise<void> {
    nav.fromHash(location.hash);
    await Promise.all([settings.load().catch(() => undefined), projects.load()]);
    booting = false;
    if (settings.value?.auto_update) void updater.checkNow(true);
  }

  $effect(() => {
    void boot();
  });
</script>

<svelte:window onhashchange={() => nav.fromHash(location.hash)} />

<div class="flex h-full flex-col bg-fc-page text-fc-fg">
  <TitleBar />
  {#if booting}
    <div class="flex flex-1 items-center justify-center"><Spinner /></div>
  {:else if settings.value && !settings.value.onboarded}
    <Onboarding />
  {:else}
    <div class="flex min-h-0 flex-1 gap-2 p-2 pt-0">
      <Sidebar />
      <main class="min-w-0 flex-1 overflow-y-auto rounded-3xl bg-fc-component p-8">
        {#if nav.view === "settings"}
          <Settings />
        {:else}
          <ProjectList />
        {/if}
      </main>
    </div>
  {/if}
</div>
