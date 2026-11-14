<script lang="ts">
  import { ForDevelopers, type DeveloperRow } from "@facile/muse";
  import type { Content } from "$lib/content/types";
  import type { Release } from "$lib/release";
  import { REPO_URL, actionUses } from "$lib/site";

  let { content, release }: { content: Content; release: Release } = $props();

  const rows: DeveloperRow[] = $derived([
    { kind: "mcp", text: content.dev.mcp, command: "fusible mcp" },
    { kind: "cli", title: content.dev.actionTitle, text: content.dev.action, command: actionUses(release.version) },
    { kind: "source", text: content.dev.source, href: REPO_URL },
  ]);
</script>

<ForDevelopers
  id="developers"
  {rows}
  description={content.dev.description}
  headingClass="text-fc-xl font-bold tracking-tight text-fc-fg sm:text-fc-2xl"
/>
