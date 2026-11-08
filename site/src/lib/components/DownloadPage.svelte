<script lang="ts">
  import { onMount } from "svelte";
  import { Button, Icon, icons } from "@facile/muse";
  import type { Content } from "$lib/content/types";
  import { parseRelease, type Release } from "$lib/release";
  import { RELEASES_URL, RELEASE_API } from "$lib/site";
  import CodeBlock from "./CodeBlock.svelte";
  import Seo from "./Seo.svelte";
  import SiteShell from "./SiteShell.svelte";
  import { scrollReveal } from "$lib/motion";

  let { content, release }: { content: Content; release: Release } = $props();

  let live = $state<Release | null>(null);
  const current = $derived(live ?? release);
  const dl = $derived(content.download);

  onMount(async () => {
    try {
      const response = await fetch(RELEASE_API, { headers: { Accept: "application/vnd.github+json" } });
      if (!response.ok) return;
      const next = parseRelease(await response.json());
      if (next.version) live = next;
    } catch {
      live = null;
    }
  });
  const ICONS = { windows: "solar:monitor-linear", macos: "solar:monitor-linear", linux: "solar:monitor-linear", android: "solar:smartphone-linear" };

  /** Formats a byte count as megabytes. */
  function megabytes(bytes: number): string {
    return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
  }
</script>

<Seo {content} meta={content.meta.download} page="/download" />
<SiteShell {content}>
  <section use:scrollReveal={{ stagger: 0.07 }} class="flex flex-col gap-6 pt-4 md:pt-10">
    <div data-reveal-item class="flex max-w-2xl flex-col gap-3">
      <h1 class="text-fc-2xl font-semibold tracking-tight text-fc-fg sm:text-fc-3xl">{dl.title}</h1>
      <p class="text-fc-md text-fc-fg-muted">{dl.intro}</p>
      {#if current.version}
        <p class="text-fc-sm text-fc-fg-muted">{dl.versionLabel}: <span class="font-fc-mono">{current.version}</span></p>
      {/if}
    </div>

    <ul class="grid grid-cols-1 gap-4 sm:grid-cols-2">
      {#each dl.platforms as platform (platform.key)}
        {@const asset = current.assets[platform.key]}
        <li
          id={platform.key}
          data-reveal-item
          class="flex flex-col justify-between gap-5 rounded-fc-lg bg-fc-surface p-6"
        >
          <div class="flex items-start gap-3">
            <span class="inline-flex size-12 items-center justify-center rounded-fc-pill bg-fc-component text-fc-fg">
              <Icon icon={ICONS[platform.key]} size={24} />
            </span>
            <div class="flex flex-col gap-0.5">
              <h2 class="text-fc-lg font-semibold text-fc-fg">{platform.title}</h2>
              <p class="text-fc-sm text-fc-fg-muted">{platform.hint}</p>
              {#if asset}
                <p class="font-fc-mono text-fc-xs text-fc-fg-muted">{asset.name} · {megabytes(asset.size)}</p>
              {/if}
            </div>
          </div>
          <div class="flex flex-wrap items-center gap-3">
            <Button variant="primary" size="md" href={asset?.url ?? RELEASES_URL} icon={icons.download}>
              {dl.button}
            </Button>
            {#if asset?.checksum}
              <a
                href={asset.checksum}
                class="text-fc-xs text-fc-fg-muted underline hover:text-fc-fg">{dl.checksum}</a
              >
            {/if}
          </div>
          {#if platform.key === "android"}
            <p class="text-fc-xs text-fc-fg-muted">{dl.androidNote}</p>
          {:else}
            <p class="text-fc-xs text-fc-fg-muted">{dl.desktopNote}</p>
          {/if}
        </li>
      {/each}
    </ul>

    <div data-reveal-item class="flex flex-col gap-3 rounded-fc-lg bg-fc-component p-6 sm:p-8">
      <h2 class="text-fc-lg font-semibold text-fc-fg">{dl.cliTitle}</h2>
      <CodeBlock code={dl.cliLines.join("\n")} {content} />
      <p class="font-fc-mono text-fc-xs text-fc-fg-muted">{dl.actionLine}</p>
    </div>

    <div data-reveal-item>
      <Button variant="ghost" size="md" class="bg-fc-surface text-fc-fg hover:bg-fc-component" href={RELEASES_URL}>
        {dl.allReleases}
      </Button>
    </div>
  </section>
</SiteShell>
