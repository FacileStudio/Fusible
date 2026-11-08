<script lang="ts">
  import { SideBar } from "@facile/muse";
  import { api } from "../lib/api";
  import { nav } from "../lib/nav.svelte";
  import UpdateBadge from "./UpdateBadge.svelte";

  let version = $state("");

  $effect(() => {
    void api.appVersion().then((v) => (version = v)).catch(() => undefined);
  });

  const pages = $derived([
    { label: "Projects", href: "#/projects", icon: "solar:folder-open-linear", active: nav.view === "projects" },
    { label: "Settings", href: "#/settings", icon: "solar:settings-linear", active: nav.view === "settings" },
  ]);
</script>

<aside class="flex w-60 shrink-0 flex-col gap-2">
  <SideBar title="Fusible" icon="solar:cpu-bolt-bold-duotone" showCollapse={false} {pages} class="min-h-0 flex-1" />
  <div class="flex items-center justify-between gap-2 rounded-fc-lg bg-fc-component px-4 py-3">
    <span class="text-fc-xs text-fc-fg-muted">{version ? `v${version}` : ""}</span>
    <UpdateBadge />
  </div>
</aside>
