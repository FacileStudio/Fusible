<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { Button, Drawer, Icon, IconButton, icons } from "@facile/muse";
  import type { Content } from "$lib/content/types";
  import { LOGO, MENU } from "$lib/icons";
  import { moveNavPill, scrollToSection, watchActiveSection } from "$lib/motion";
  import LangSwitch from "../LangSwitch.svelte";
  import ThemeButton from "../ThemeButton.svelte";

  let { content }: { content: Content } = $props();

  const home = $derived(content.prefix || "/");
  const path = $derived(page.url.pathname.replace(/\/$/, ""));
  const onHome = $derived(path === content.prefix);
  const links = $derived([
    { key: "features", label: content.nav.features, href: `${home}#features`, id: "features" },
    { key: "agents", label: content.nav.agents, href: `${home}#agents`, id: "agents" },
    { key: "mobile", label: content.nav.mobile, href: `${home}#mobile`, id: "mobile" },
    { key: "docs", label: content.nav.docs, href: `${content.prefix}/docs`, id: "" },
    { key: "download", label: content.nav.download, href: `${content.prefix}/download`, id: "" },
  ]);

  let scrolled = $state(false);
  let section = $state("");
  let open = $state(false);
  let pill: HTMLSpanElement | null = $state(null);
  let els: Record<string, HTMLAnchorElement> = $state({});

  const active = $derived(path.endsWith("/docs") ? "docs" : path.endsWith("/download") ? "download" : section);

  $effect(() => moveNavPill(pill, els[active], true));

  onMount(() => {
    moveNavPill(pill, els[active], false);
    const ids = ["features", "agents", "mobile"];
    return watchActiveSection(ids, (id) => (section = id), (s) => (scrolled = s));
  });

  function go(event: MouseEvent, id: string) {
    if (id && onHome) scrollToSection(id, event);
    open = false;
  }
</script>

<a
  href="#main"
  class="sr-only focus:not-sr-only focus:fixed focus:left-4 focus:top-4 focus:z-50 focus:rounded-fc-pill
    focus:bg-fc-accent focus:px-4 focus:py-2 focus:text-fc-accent-fg">{content.ui.skip}</a
>

<div class="sticky top-0 z-30 flex w-full justify-center pointer-events-none px-3 sm:px-6">
  <header
    class="pointer-events-auto flex h-14 w-full items-center justify-between gap-3 transition-all duration-500 ease-fc
      {scrolled
      ? 'mt-3 max-w-4xl rounded-fc-pill bg-fc-surface/65 px-4 sm:px-6 backdrop-blur-xl'
      : 'mt-0 max-w-fc-xl rounded-none bg-fc-page/80 px-2 sm:px-4 backdrop-blur-md'}"
  >
    <a
      href={home}
      class="inline-flex items-center gap-2 rounded-fc-pill px-2 py-1 text-fc-md font-semibold text-fc-fg
        focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-fc-ring"
    >
      <Icon icon={LOGO} size={22} />
      <span>Fusible</span>
    </a>

    <nav aria-label={content.ui.primaryNav} class="relative hidden md:flex items-center gap-0.5 p-1">
      <span
        bind:this={pill}
        aria-hidden="true"
        class="pointer-events-none absolute inset-y-1 left-0 rounded-fc-pill bg-fc-accent opacity-0"
      ></span>
      {#each links as link (link.key)}
        <a
          bind:this={els[link.key]}
          href={link.href}
          aria-current={active === link.key ? "page" : undefined}
          onclick={(e) => go(e, link.id)}
          class="relative z-10 inline-flex h-8 items-center whitespace-nowrap rounded-fc-pill px-3 text-fc-xs
            font-medium transition-colors duration-200 focus-visible:outline-2 focus-visible:outline-offset-2
            focus-visible:outline-fc-ring {active === link.key
            ? 'text-fc-accent-fg'
            : 'text-fc-fg-muted hover:text-fc-fg'}">{link.label}</a
        >
      {/each}
    </nav>

    <div class="flex items-center gap-2">
      <div class="hidden sm:flex items-center gap-2">
        <LangSwitch {content} />
        <ThemeButton {content} />
      </div>
      <Button variant="primary" size="md" class="hidden lg:inline-flex h-9 px-4" href="{content.prefix}/download" icon={icons.download}>
        {content.hero.download}
      </Button>
      <IconButton
        variant="ghost"
        class="size-9 pointer-coarse:size-11 border-0 bg-fc-surface/70 text-fc-fg hover:bg-fc-surface md:hidden"
        aria-label={content.ui.openMenu}
        aria-expanded={open}
        onclick={() => (open = true)}
      >
        <Icon icon={MENU} size={18} />
      </IconButton>
    </div>
  </header>
</div>

<Drawer bind:open title={content.ui.menuTitle} showClose>
  <nav aria-label={content.ui.menuTitle} class="flex flex-col gap-1.5">
    {#each links as link (link.key)}
      <a
        href={link.href}
        onclick={(e) => go(e, link.id)}
        class="inline-flex h-11 items-center rounded-fc-pill px-4 text-fc-sm font-medium transition-colors
          {active === link.key ? 'bg-fc-accent text-fc-accent-fg' : 'text-fc-fg-muted hover:bg-fc-surface hover:text-fc-fg'}"
        >{link.label}</a
      >
    {/each}
  </nav>
  <div class="flex items-center gap-2 pt-4 pb-2">
    <LangSwitch {content} />
    <ThemeButton {content} />
  </div>
</Drawer>
