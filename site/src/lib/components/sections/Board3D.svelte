<script lang="ts">
  import { Section } from "@facile/muse";
  import type { Content } from "$lib/content/types";
  import { scrollReveal } from "$lib/motion";
  import PcbViewer from "../PcbViewer.svelte";

  let { content }: { content: Content } = $props();
</script>

<div use:scrollReveal={{ y: 20 }} class="pb-12">
  <Section id="board" title={content.demos.board.title} description={content.demos.board.text}>
    <PcbViewer copy={content.demos.board} />
  </Section>
</div>
