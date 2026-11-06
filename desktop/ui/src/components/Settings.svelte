<script lang="ts">
  import { Alert, Tabs } from "@facile/muse";
  import { nav, type SettingsTab } from "../lib/nav.svelte";
  import { settings } from "../lib/settings.svelte";
  import General from "./settings/General.svelte";
  import Phone from "./settings/Phone.svelte";
  import Toolchain from "./settings/Toolchain.svelte";
  import Updates from "./settings/Updates.svelte";

  const items = [
    { id: "general", label: "General" },
    { id: "toolchain", label: "Toolchain" },
    { id: "updates", label: "Updates" },
    { id: "phone", label: "Phone" },
  ];
</script>

<section class="mx-auto flex max-w-3xl flex-col gap-6">
  <header class="flex items-center justify-between gap-4">
    <h1 class="text-fc-2xl font-semibold">Settings</h1>
    <p class="text-fc-sm text-fc-fg-muted" role="status" aria-live="polite">{settings.saved ? "Saved" : ""}</p>
  </header>
  <Tabs {items} value={nav.tab} label="Settings sections" onChange={(id) => (nav.tab = id as SettingsTab)} />
  {#if settings.error}<Alert tone="danger">{settings.error}</Alert>{/if}
  {#if settings.value}
    {#if nav.tab === "general"}
      <General />
    {:else if nav.tab === "toolchain"}
      <Toolchain />
    {:else if nav.tab === "updates"}
      <Updates />
    {:else}
      <Phone />
    {/if}
  {/if}
</section>
