<script lang="ts">
  import { page } from "$app/state";
  import type { Content } from "$lib/content/types";
  import { LOCALE_PREFIXES, localeHref, type SiteLocale } from "$lib/site";

  let { content }: { content: Content } = $props();

  const codes = Object.keys(LOCALE_PREFIXES) as SiteLocale[];
  const options = $derived(
    codes.map((code) => ({ code, label: code.toUpperCase(), href: localeHref(code, page.url.pathname) })),
  );
</script>

<div role="group" aria-label={content.ui.language} class="inline-flex gap-1 rounded-fc-pill bg-fc-surface p-1">
  {#each options as option (option.code)}
    <a
      href={option.href}
      hreflang={option.code}
      lang={option.code}
      aria-current={option.code === content.locale ? "true" : undefined}
      data-sveltekit-reload
      class="inline-flex h-7 min-w-9 items-center justify-center rounded-fc-pill px-2.5 text-fc-xs font-medium
        transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-fc-ring
        {option.code === content.locale ? 'bg-fc-accent text-fc-accent-fg' : 'text-fc-fg-muted hover:text-fc-fg'}"
      >{option.label}</a
    >
  {/each}
</div>
