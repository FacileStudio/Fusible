import type { PageServerLoad } from "./$types";
import { fetchRelease } from "$lib/release.server";

export const prerender = true;

export const load: PageServerLoad = async ({ fetch }) => ({ release: await fetchRelease(fetch) });
