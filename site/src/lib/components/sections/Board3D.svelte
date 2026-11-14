<script lang="ts">
  import { Button, Section, icons } from "@facile/muse";
  import type { Content } from "$lib/content/types";
  import { PASTILLE_URL } from "$lib/content/shared";
  import { scrollReveal } from "$lib/motion";
  import PcbViewer from "../PcbViewer.svelte";

  let { content }: { content: Content } = $props();
</script>

<div use:scrollReveal={{ y: 20 }} class="pb-12">
  <Section id="board" title={content.demos.board.title} description={content.demos.board.text}>
    {#snippet actions()}
      <Button variant="secondary" size="sm" href={PASTILLE_URL} target="_blank" rel="noreferrer" icon={icons.external}>
        {content.demos.board.link}
      </Button>
    {/snippet}
    <PcbViewer copy={content.demos.board} />
  </Section>
</div>
