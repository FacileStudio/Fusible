<script lang="ts">
  import type { Snippet } from "svelte";
  import { page } from "$app/state";
  import { Button, LandingFooter, LandingHeader, Page, icons, provideLocale } from "@facile/muse";
  import type { Content } from "$lib/content/types";
  import { LOGO } from "$lib/icons";
  import { CRATES_URL, REPO_URL } from "$lib/site";
  import LangSwitch from "./LangSwitch.svelte";
  import ThemeButton from "./ThemeButton.svelte";

  let { content, children }: { content: Content; children: Snippet } = $props();

  provideLocale(() => content.locale);

  const home = $derived(content.prefix || "/");
  const path = $derived(page.url.pathname.replace(/\/$/, ""));
  const links = $derived([
    { label: content.nav.features, href: `${home}#features` },
    { label: content.nav.agents, href: `${home}#agents` },
    { label: content.nav.mobile, href: `${home}#mobile` },
    { label: content.nav.docs, href: `${content.prefix}/docs`, active: path.endsWith("/docs") },
    { label: content.nav.download, href: `${content.prefix}/download`, active: path.endsWith("/download") },
  ]);
  const columns = $derived([
    {
      title: "Fusible",
      links: [
        { label: content.nav.docs, href: `${content.prefix}/docs` },
        { label: content.nav.download, href: `${content.prefix}/download` },
      ],
    },
    {
      title: content.footer.license,
      links: [
        { label: content.footer.github, href: REPO_URL, external: true },
        { label: content.footer.crates, href: CRATES_URL, external: true },
        { label: content.footer.studio, href: "https://facile.studio", external: true },
      ],
    },
  ]);
</script>

<div class="flex min-h-dvh w-full flex-col bg-fc-page text-fc-fg">
  <LandingHeader appName="Fusible" icon={LOGO} homeHref={home} {links} skipHref="#main">
    {#snippet actions()}
      <LangSwitch {content} />
      <ThemeButton {content} />
      <Button size="sm" class="h-9" href="{content.prefix}/download" icon={icons.download}>{content.hero.download}</Button>
    {/snippet}
  </LandingHeader>
  <main id="main" class="flex-1"><Page width="xl">{@render children()}</Page></main>
  <LandingFooter appName="Fusible" icon={LOGO} homeHref={home} tagline={content.hero.title} {columns} />
</div>
