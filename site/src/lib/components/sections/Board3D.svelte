<script lang="ts">
  import type { Content } from "$lib/content/types";
  import PcbViewer from "../PcbViewer.svelte";

  let { content }: { content: Content } = $props();
</script>

<section id="board" aria-labelledby="board-title" class="flex flex-col gap-6">
  <div class="flex flex-col gap-2">
    <h2 id="board-title" class="text-fc-xl font-bold tracking-tight text-fc-fg sm:text-fc-2xl">{content.demos.board.title}</h2>
    <p class="text-fc-md text-fc-fg-muted">{content.demos.board.text}</p>
  </div>
  <PcbViewer copy={content.demos.board} />
</section>
