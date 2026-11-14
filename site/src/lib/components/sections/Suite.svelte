<script lang="ts">
  import { Icon, WorksWith, icons, type WorksWithApp } from "@facile/muse";
  import type { Content } from "$lib/content/types";
  import { BULLE_URL, NUAGE_URL, PASTILLE_URL, SUITE_URL } from "$lib/content/shared";
  import { scrollReveal } from "$lib/motion";

  let { content }: { content: Content } = $props();

  const apps: WorksWithApp[] = $derived([
    { name: "Nuage", icon: "solar:cloud-bold-duotone", line: content.suite.nuage, href: NUAGE_URL },
    { name: "Bulle", icon: "solar:smile-circle-bold-duotone", line: content.suite.bulle, href: BULLE_URL },
    { name: "Pastille", icon: "solar:circle-dot-bold-duotone", line: content.suite.pastille, href: PASTILLE_URL },
  ]);
</script>

<div use:scrollReveal={{ y: 20 }} class="flex flex-col gap-4">
  <WorksWith id="suite" {apps} description={content.suite.description} headingClass="text-fc-xl font-bold tracking-tight text-fc-fg sm:text-fc-2xl" />
  <p class="text-fc-sm text-fc-fg-muted">
    {content.suite.suite}
    <a
      href={SUITE_URL}
      class="inline-flex items-center gap-1 font-medium text-fc-fg underline-offset-2 hover:underline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-fc-ring"
      >{content.suite.suiteLink}<Icon icon={icons.external} size={14} /></a
    >
  </p>
</div>
