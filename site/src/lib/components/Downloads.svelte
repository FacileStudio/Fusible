<script lang="ts">
  import { onMount } from "svelte";
  import { DownloadSection } from "@facile/muse";
  import type { Content } from "$lib/content/types";
  import { parseRelease, type Release } from "$lib/release";
  import { RELEASES_URL, RELEASE_API } from "$lib/site";

  let { content, release, headingLevel = 3 }: { content: Content; release: Release; headingLevel?: 2 | 3 | 4 } =
    $props();

  let live = $state<Release | null>(null);
  const current = $derived(live ?? release);
  const commands = $derived(content.download.cli);

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
</script>

<DownloadSection downloads={current.downloads} {commands} releasesHref={RELEASES_URL} {headingLevel} />
