<script lang="ts">
  import { onMount } from "svelte";
  import { Spinner } from "@facile/muse";
  import { mountGlb } from "$lib/board3d";
  import { useT } from "$lib/i18n";

  let { load }: { load: () => Promise<ArrayBuffer> } = $props();
  const t = useT();
  let target: HTMLElement | undefined = $state();
  let phase = $state<"loading" | "ready" | "failed">("loading");

  onMount(() => {
    let stop: (() => void) | undefined;
    let gone = false;
    load()
      .then((glb) => mountGlb(target!, glb))
      .then((dispose) => {
        if (gone) dispose();
        else [stop, phase] = [dispose, "ready"];
      })
      .catch(() => (phase = "failed"));
    return () => {
      gone = true;
      stop?.();
    };
  });
</script>

<div class="relative h-[60dvh] overflow-hidden rounded-3xl bg-fc-surface">
  <div bind:this={target} class="absolute inset-0"></div>
  {#if phase !== "ready"}
    <div class="absolute inset-0 flex items-center justify-center gap-2 text-fc-fg-muted">
      {#if phase === "loading"}<Spinner /> {t("loading")}{:else}{t("nothingToShow")}{/if}
    </div>
  {/if}
</div>
