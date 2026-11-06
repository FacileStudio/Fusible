<script lang="ts">
  import "../app.css";
  import { page } from "$app/state";
  import { MobileNav } from "@facile/muse";
  import { initIcons, ICON } from "$lib/icons";
  import { initLocale, useT } from "$lib/i18n";

  let { children } = $props();

  initIcons();
  initLocale();
  const t = useT();

  const id = $derived(page.url.searchParams.get("id") ?? "");
  const path = $derived(page.url.pathname.replace(/\/$/, ""));
  const tabs = [
    { to: "/host", label: "navOverview", icon: ICON.overview },
    { to: "/host/board", label: "navBoards", icon: ICON.boards },
    { to: "/host/checks", label: "navChecks", icon: ICON.checks },
    { to: "/host/bom", label: "navBom", icon: ICON.bom },
    { to: "/host/bodies", label: "navBodies", icon: ICON.bodies },
  ] as const;
  const items = $derived(
    tabs.map((tab) => ({
      href: `${tab.to}?id=${encodeURIComponent(id)}`,
      label: t(tab.label),
      icon: tab.icon,
      active: path === tab.to,
    })),
  );
  const inHost = $derived(path.startsWith("/host") && id !== "");
</script>

<div class="min-h-dvh bg-fc-bg text-fc-fg">
  {@render children()}
  {#if inHost}
    <div class="fixed inset-x-0 bottom-0 z-10 flex justify-center px-4 pb-[max(1rem,env(safe-area-inset-bottom))]">
      <MobileNav {items} class="md:flex" />
    </div>
  {/if}
</div>
