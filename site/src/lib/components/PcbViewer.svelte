<script lang="ts">
  import { onMount } from "svelte";
  import { Button, Spinner } from "@facile/muse";
  import type { BoardContent } from "$lib/content/types";
  import { whenNear } from "$lib/motion";
  import type { Preset, SceneControl } from "$lib/pcb-scene";

  let { copy }: { copy: BoardContent } = $props();

  const PRESETS: Preset[] = ["iso", "top", "side"];
  const TOUCH = "pointer-coarse:h-11";
  const PILL = "flex flex-wrap items-center gap-1 rounded-fc-pill bg-fc-surface/90 p-1 backdrop-blur pointer-events-auto";

  let host: HTMLDivElement;
  let status = $state<"idle" | "loading" | "ready" | "error">("idle");
  let preset = $state<Preset>("iso");
  let rotating = $state(true);
  let interactive = $state(false);
  let viewer: SceneControl | null = null;

  /**
   * Returns the button classes for a toggle in its on or off state.
   */
  function toggleClass(on: boolean) {
    return on ? TOUCH : `${TOUCH} text-fc-fg hover:bg-fc-component`;
  }

  /**
   * Moves the camera to a preset angle and pauses auto-rotation.
   */
  function choosePreset(next: Preset) {
    preset = next;
    rotating = false;
    viewer?.setRotating(false);
    viewer?.applyPreset(next);
  }

  /**
   * Toggles turntable auto-rotation.
   */
  function toggleRotate() {
    rotating = !rotating;
    viewer?.setRotating(rotating);
  }

  /**
   * Turns drag-to-orbit and zoom on or off, so the page scrolls normally until the visitor opts in.
   */
  function setInteractive(on: boolean) {
    interactive = on;
    viewer?.setInteractive(on);
  }

  onMount(() => {
    rotating = !window.matchMedia("(prefers-reduced-motion: reduce)").matches;
    let disposed = false;
    const enable = () => setInteractive(true);

    async function boot() {
      if (disposed || status !== "idle") return;
      status = "loading";
      try {
        const { mountPcbScene } = await import("$lib/pcb-scene");
        const ctrl = await mountPcbScene(host, rotating, {
          onUserInteract: () => (rotating = false),
          onHidden: () => (interactive = false),
        });
        if (disposed) {
          ctrl.dispose();
          return;
        }
        viewer = ctrl;
        host.addEventListener("click", enable);
        status = "ready";
      } catch {
        if (!disposed) status = "error";
      }
    }

    const stopNear = whenNear(host, boot);

    return () => {
      disposed = true;
      stopNear();
      host.removeEventListener("click", enable);
      viewer?.dispose();
      viewer = null;
    };
  });
</script>

<div class="relative h-[380px] w-full overflow-hidden rounded-fc-lg bg-fc-component sm:h-[480px] lg:h-[560px]">
  <div bind:this={host} class="absolute inset-0" aria-hidden="true"></div>

  {#if status === "ready"}
    <div class="pointer-events-none absolute top-0 right-0 left-0 flex flex-wrap items-center justify-end gap-2 p-4">
      <div class={PILL}>
        {#each PRESETS as option (option)}
          <Button
            size="sm"
            variant={preset === option ? "primary" : "ghost"}
            class={toggleClass(preset === option)}
            aria-pressed={preset === option}
            onclick={() => choosePreset(option)}
          >
            {copy.presets[option]}
          </Button>
        {/each}
        <Button
          size="sm"
          variant={rotating ? "primary" : "ghost"}
          class={toggleClass(rotating)}
          aria-pressed={rotating}
          onclick={toggleRotate}
        >
          {copy.auto}
        </Button>
      </div>
    </div>
  {/if}

  {#if status === "idle" || status === "loading"}
    <div class="pointer-events-none absolute inset-0 flex flex-col items-center justify-center gap-3 bg-fc-component/80">
      <Spinner />
      <span class="font-fc-mono text-fc-xs text-fc-fg-muted">{copy.loading}</span>
    </div>
  {:else if status === "error"}
    <div class="absolute inset-0 flex flex-col items-center justify-center gap-2 p-6 text-center">
      <span class="text-fc-sm font-bold text-fc-fg">{copy.failed}</span>
      <span class="text-fc-xs text-fc-fg-muted">{copy.failedText}</span>
    </div>
  {:else}
    <div class="pointer-events-none absolute right-0 bottom-0 left-0 flex flex-wrap items-center justify-between gap-2 p-4">
      <div class="pointer-events-auto flex flex-wrap items-center gap-2">
        <Button
          size="sm"
          variant={interactive ? "primary" : "ghost"}
          class={interactive ? TOUCH : `${TOUCH} bg-fc-surface/85 text-fc-fg backdrop-blur hover:bg-fc-surface`}
          onclick={() => setInteractive(!interactive)}
        >
          {interactive ? copy.done : copy.explore}
        </Button>
        {#if interactive}
          <span class="rounded-fc-pill bg-fc-surface/85 px-3 py-1 font-fc-mono text-fc-xs text-fc-fg-muted backdrop-blur">
            {copy.hint}
          </span>
        {/if}
      </div>
      <span class="hidden rounded-fc-pill bg-fc-surface/85 px-3 py-1 font-fc-mono text-fc-xs text-fc-success backdrop-blur sm:inline-flex">
        {copy.status}
      </span>
    </div>
  {/if}
</div>
