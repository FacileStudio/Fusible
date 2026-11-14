import { LOCALE_PREFIXES, PAGES, localesFor, pageUrl } from "$lib/site";

export const prerender = true;

/** Serves the sitemap with one entry per page and locale, each carrying hreflang alternates. */
export function GET() {
  const lastmod = new Date().toISOString().split("T")[0];
  const entry = (prefix: string, page: string) => {
    const links = [
      ...localesFor(page).map((lang) => ({ lang, href: pageUrl(LOCALE_PREFIXES[lang], page) })),
      { lang: "x-default", href: pageUrl("", page) },
    ]
      .map((l) => `    <xhtml:link rel="alternate" hreflang="${l.lang}" href="${l.href}"/>`)
      .join("\n");
    return `  <url>\n    <loc>${pageUrl(prefix, page)}</loc>\n    <lastmod>${lastmod}</lastmod>\n${links}\n  </url>`;
  };
  const urls = PAGES.flatMap((page) => localesFor(page).map((lang) => entry(LOCALE_PREFIXES[lang], page)))
    .join("\n");
  const xml =
    '<?xml version="1.0" encoding="UTF-8"?>\n' +
    '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml">\n' +
    `${urls}\n</urlset>\n`;
  return new Response(xml, { headers: { "Content-Type": "application/xml" } });
}
