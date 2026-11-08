<script lang="ts">
  import { Alert, SettingsLayout } from "@facile/muse";
  import { nav, type SettingsTab } from "../lib/nav.svelte";
  import { settings } from "../lib/settings.svelte";
  import General from "./settings/General.svelte";
  import Phone from "./settings/Phone.svelte";
  import Toolchain from "./settings/Toolchain.svelte";
  import Updates from "./settings/Updates.svelte";

  const sections: { id: SettingsTab; label: string; icon: string; href: string }[] = [
    { id: "general", label: "General", icon: "solar:settings-linear", href: "#/settings/general" },
    { id: "toolchain", label: "Toolchain", icon: "solar:cpu-bolt-linear", href: "#/settings/toolchain" },
    { id: "updates", label: "Updates", icon: "solar:refresh-linear", href: "#/settings/updates" },
    { id: "phone", label: "Phone", icon: "solar:smartphone-linear", href: "#/settings/phone" },
  ];
</script>

<SettingsLayout {sections} active={nav.tab} backHref="#/settings" title="Settings" ordered={false} class="mx-auto max-w-4xl">
  {#snippet children(section)}
    <div class="flex flex-col gap-4">
      {#if settings.error}<Alert tone="danger">{settings.error}</Alert>{/if}
      <p class="sr-only" role="status" aria-live="polite">{settings.saved ? "Saved" : ""}</p>
      {#if settings.value}
        {#if section === "general"}
          <General />
        {:else if section === "toolchain"}
          <Toolchain />
        {:else if section === "updates"}
          <Updates />
        {:else}
          <Phone />
        {/if}
      {/if}
    </div>
  {/snippet}
</SettingsLayout>
