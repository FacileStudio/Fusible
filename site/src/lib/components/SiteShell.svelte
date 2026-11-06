<script lang="ts">
  import type { Snippet } from "svelte";
  import { Page, provideLocale } from "@facile/muse";
  import type { Content } from "$lib/content/types";
  import Footer from "./sections/Footer.svelte";
  import Nav from "./sections/Nav.svelte";

  let { content, children }: { content: Content; children: Snippet } = $props();

  provideLocale(() => content.locale);
</script>

<div class="flex min-h-dvh w-full flex-col bg-fc-page text-fc-fg">
  <Nav {content} />
  <main id="main" class="flex-1"><Page width="xl">{@render children()}</Page></main>
  <Footer {content} />
</div>
