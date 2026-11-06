<script lang="ts">
  import type { Content } from "$lib/content/types";
  import { scrollToSection } from "$lib/motion";
  import CodeBlock from "./CodeBlock.svelte";
  import Seo from "./Seo.svelte";
  import SiteShell from "./SiteShell.svelte";

  let { content }: { content: Content } = $props();

  const docs = $derived(content.docs);
</script>

<Seo {content} meta={content.meta.docs} page="/docs" />
<SiteShell {content}>
  <div class="grid grid-cols-1 gap-8 pt-4 md:pt-10 lg:grid-cols-[14rem_1fr] lg:gap-12">
    <aside class="lg:sticky lg:top-24 lg:self-start">
      <nav aria-label={docs.toc} class="flex flex-col gap-2">
        <p class="px-1 text-fc-xs font-medium uppercase text-fc-fg-muted">{docs.toc}</p>
        <ul class="flex flex-wrap gap-1.5 lg:flex-col lg:gap-1">
          {#each docs.sections as section (section.id)}
            <li>
              <a
                href="#{section.id}"
                onclick={(e) => scrollToSection(section.id, e)}
                class="inline-flex h-8 items-center rounded-full bg-fc-surface px-3.5 text-fc-xs font-medium
                  text-fc-fg-muted transition-colors hover:text-fc-fg focus-visible:outline-2
                  focus-visible:outline-offset-2 focus-visible:outline-fc-ring">{section.title}</a
              >
            </li>
          {/each}
        </ul>
      </nav>
    </aside>

    <article class="flex min-w-0 flex-col gap-10">
      <header class="flex max-w-2xl flex-col gap-3">
        <h1 class="text-fc-2xl font-semibold tracking-tight text-fc-fg sm:text-fc-3xl">{docs.title}</h1>
        <p class="text-fc-md text-fc-fg-muted">{docs.intro}</p>
      </header>
      {#each docs.sections as section (section.id)}
        <section id={section.id} aria-labelledby="{section.id}-title" class="flex scroll-mt-24 flex-col gap-4">
          <h2 id="{section.id}-title" class="text-fc-xl font-semibold tracking-tight text-fc-fg">{section.title}</h2>
          {#each section.blocks as block, i (i)}
            {#if block.kind === "p"}
              <p class="max-w-2xl text-fc-sm text-fc-fg-muted">{block.text}</p>
            {:else if block.kind === "code"}
              <CodeBlock code={block.code} {content} />
            {:else if block.kind === "list"}
              <ul class="flex max-w-2xl list-disc flex-col gap-1.5 pl-5 text-fc-sm text-fc-fg-muted">
                {#each block.items as item (item)}<li>{item}</li>{/each}
              </ul>
            {:else}
              <div class="overflow-x-auto rounded-fc-md bg-fc-surface p-2">
                <table class="w-full text-left text-fc-sm">
                  <thead>
                    <tr>
                      {#each block.head as cell (cell)}
                        <th scope="col" class="px-3 py-2 text-fc-xs font-medium uppercase text-fc-fg-muted">{cell}</th>
                      {/each}
                    </tr>
                  </thead>
                  <tbody>
                    {#each block.rows as row (row[0])}
                      <tr>
                        {#each row as cell, c (c)}
                          <td class="px-3 py-2.5 align-top {c === 0 ? 'font-mono text-fc-xs text-fc-fg' : 'text-fc-fg-muted'}"
                            >{cell}</td
                          >
                        {/each}
                      </tr>
                    {/each}
                  </tbody>
                </table>
              </div>
            {/if}
          {/each}
        </section>
      {/each}
    </article>
  </div>
</SiteShell>
