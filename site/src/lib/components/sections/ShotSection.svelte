<script lang="ts">
  import { Button } from "@facile/muse";
  import type { Content, ShotContent } from "$lib/content/types";
  import { scrollReveal } from "$lib/motion";

  let {
    id,
    data,
    content,
    src,
    width,
    height,
    phone = false,
    reverse = false,
    icon,
  }: {
    id: string;
    data: ShotContent;
    content: Content;
    src: string;
    width: number;
    height: number;
    phone?: boolean;
    reverse?: boolean;
    icon?: string;
  } = $props();

  const href = $derived(data.cta?.href.startsWith("/") ? `${content.prefix}${data.cta.href}` : data.cta?.href);
</script>

<section
  {id}
  aria-labelledby="{id}-title"
  use:scrollReveal={{ stagger: 0.1 }}
  class="grid grid-cols-1 items-center gap-8 lg:grid-cols-2 lg:gap-14"
>
  <div data-reveal-item class="flex flex-col items-start gap-4 {reverse ? 'lg:order-2' : ''}">
    <h2 id="{id}-title" class="text-fc-xl font-semibold tracking-tight text-fc-fg sm:text-fc-2xl">{data.title}</h2>
    <p class="max-w-md text-fc-md text-fc-fg-muted">{data.text}</p>
    {#if data.cta && href}
      <Button variant="primary" size="lg" {href} {icon}>{data.cta.label}</Button>
    {/if}
  </div>
  <div data-reveal-item class="flex justify-center {reverse ? 'lg:order-1' : ''}">
    {#if phone}
      <div class="w-64 rounded-[2.5rem] bg-fc-component p-3">
        <img {src} alt={data.alt} {width} {height} loading="lazy" class="h-auto w-full rounded-[1.75rem]" />
      </div>
    {:else}
      <div class="w-full rounded-fc-lg bg-fc-surface p-2">
        <img {src} alt={data.alt} {width} {height} loading="lazy" class="h-auto w-full rounded-fc-xs" />
      </div>
    {/if}
  </div>
</section>
