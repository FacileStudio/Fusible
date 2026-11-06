import type { Handle } from "@sveltejs/kit";

/** Writes the page language into the root element so French pages ship lang="fr". */
export const handle: Handle = ({ event, resolve }) => {
  const path = event.url.pathname;
  const lang = path === "/fr" || path.startsWith("/fr/") ? "fr" : "en";
  return resolve(event, { transformPageChunk: ({ html }) => html.replace("%lang%", lang) });
};
