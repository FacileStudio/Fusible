import type { Handle } from "@sveltejs/kit";
import { splitPath } from "$lib/site";

/** Writes the page language into the root element so each locale ships its own lang attribute. */
export const handle: Handle = ({ event, resolve }) => {
  const lang = splitPath(event.url.pathname).locale;
  return resolve(event, { transformPageChunk: ({ html }) => html.replace("%lang%", lang) });
};
