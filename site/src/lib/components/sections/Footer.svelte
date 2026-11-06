<script lang="ts">
  import { Icon } from "@facile/muse";
  import type { Content } from "$lib/content/types";
  import { CRATES_URL, REPO_URL } from "$lib/site";
  import { GITHUB, LOGO } from "$lib/icons";

  let { content }: { content: Content } = $props();
</script>

<footer class="mx-auto w-full max-w-fc-xl px-4 pb-10 pt-8 sm:px-6 md:px-10">
  <div
    class="flex flex-col items-center justify-between gap-4 rounded-fc-lg bg-fc-component px-6 py-5
      text-fc-xs text-fc-fg-muted sm:flex-row"
  >
    <div class="flex flex-wrap items-center gap-2.5">
      <span class="inline-flex items-center gap-1.5 font-semibold text-fc-fg">
        <Icon icon={LOGO} size={16} />
        <span>Fusible</span>
      </span>
      <span aria-hidden="true">·</span>
      <span>{content.footer.license}</span>
    </div>
    <div class="flex flex-wrap items-center gap-4">
      <a href={REPO_URL} target="_blank" rel="noreferrer" class="inline-flex items-center gap-1.5 hover:text-fc-fg">
        <Icon icon={GITHUB} size={14} />
        {content.footer.github}
      </a>
      <a href={CRATES_URL} target="_blank" rel="noreferrer" class="hover:text-fc-fg">{content.footer.crates}</a>
      <a href="https://facile.studio" target="_blank" rel="noreferrer" class="hover:text-fc-fg"
        >{content.footer.studio}</a
      >
    </div>
  </div>
</footer>
