<script lang="ts">
  import type { Content, Meta } from "$lib/content/types";
  import { SITE_URL, pageUrl } from "$lib/site";

  let { content, meta, page, jsonLd = false }: { content: Content; meta: Meta; page: string; jsonLd?: boolean } =
    $props();

  const url = $derived(pageUrl(content.prefix, page));
  const alternates = $derived([
    { lang: "en", href: pageUrl("", page) },
    { lang: "fr", href: pageUrl("/fr", page) },
    { lang: "x-default", href: pageUrl("", page) },
  ]);
  const altLocale = $derived(content.locale === "en" ? "fr_FR" : "en_US");
  const image = `${SITE_URL}/og.png`;
  const structured = $derived(
    JSON.stringify({
      "@context": "https://schema.org",
      "@type": "SoftwareApplication",
      name: "Fusible",
      applicationCategory: "DeveloperApplication",
      operatingSystem: "Windows, macOS, Linux, Android",
      description: meta.description,
      url: SITE_URL,
      downloadUrl: `${SITE_URL}/download`,
      softwareVersion: "1.0.0",
      license: "https://opensource.org/licenses/MIT",
      isAccessibleForFree: true,
      offers: { "@type": "Offer", price: "0", priceCurrency: "EUR" },
      author: { "@type": "Organization", name: "Facile Studio", url: "https://facile.studio" },
      codeRepository: "https://github.com/FacileStudio/Fusible",
    }).replace(/</g, "\\u003c"),
  );
</script>

<svelte:head>
  <title>{meta.title}</title>
  <meta name="description" content={meta.description} />
  <link rel="canonical" href={url} />
  {#each alternates as alt (alt.lang)}
    <link rel="alternate" hreflang={alt.lang} href={alt.href} />
  {/each}
  <meta property="og:type" content="website" />
  <meta property="og:site_name" content="Fusible" />
  <meta property="og:url" content={url} />
  <meta property="og:title" content={meta.title} />
  <meta property="og:description" content={meta.description} />
  <meta property="og:image" content={image} />
  <meta property="og:image:width" content="1200" />
  <meta property="og:image:height" content="630" />
  <meta property="og:image:alt" content={meta.imageAlt} />
  <meta property="og:locale" content={content.ogLocale} />
  <meta property="og:locale:alternate" content={altLocale} />
  <meta name="twitter:card" content="summary_large_image" />
  <meta name="twitter:title" content={meta.title} />
  <meta name="twitter:description" content={meta.description} />
  <meta name="twitter:image" content={image} />
  {#if jsonLd}
    {@html `<script type="application/ld+json">${structured}</script>`}
  {/if}
</svelte:head>
