<script lang="ts">
  import { SideBar } from "@facile/muse";
  import { nav } from "../lib/nav.svelte";

  const pages = $derived([
    { label: "Projects", href: "#/projects", icon: "solar:folder-open-linear", active: nav.view === "projects" },
    { label: "Settings", href: "#/settings", icon: "solar:settings-linear", active: nav.view === "settings" },
  ]);
</script>

<aside class="flex w-60 shrink-0 flex-col">
  <SideBar title="Fusible" icon="solar:cpu-bolt-bold-duotone" showCollapse={false} {pages} class="min-h-0 flex-1" />
</aside>
