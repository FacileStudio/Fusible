<script lang="ts">
  import { Button } from "@facile/muse";
  import { gsap } from "gsap";
  import { prefersReducedMotion } from "@facile/muse";
  import { settings } from "../../lib/settings.svelte";
  import Agents from "./Agents.svelte";
  import FirstProject from "./FirstProject.svelte";
  import Phone from "./Phone.svelte";
  import Toolchain from "./Toolchain.svelte";
  import Welcome from "./Welcome.svelte";

  const steps = [Welcome, Toolchain, FirstProject, Phone, Agents];

  let index = $state(0);
  let direction = 1;
  let panel = $state<HTMLElement | null>(null);

  const last = $derived(index === steps.length - 1);
  const Step = $derived(steps[index]);

  $effect(() => {
    if (!panel || prefersReducedMotion()) return;
    gsap.fromTo(panel, { x: 40 * direction, opacity: 0 }, { x: 0, opacity: 1, duration: 0.3, ease: "power2.out" });
  });

  async function next(): Promise<void> {
    if (last) return void (await settings.save({ onboarded: true }));
    direction = 1;
    index += 1;
  }
</script>

<div class="flex min-h-0 flex-1 items-center justify-center p-6">
  <div class="flex w-full max-w-xl flex-col gap-8 overflow-hidden rounded-3xl bg-fc-component p-10">
    <div bind:this={panel} class="flex flex-col gap-6" aria-live="polite">
      <Step />
    </div>
    <footer class="flex items-center justify-between gap-4">
      <p class="text-fc-xs text-fc-fg-muted">Step {index + 1} of {steps.length}</p>
      <div class="flex items-center gap-2">
        <Button variant="ghost" onclick={next}>Skip</Button>
        <Button onclick={next}>{last ? "Finish" : "Continue"}</Button>
      </div>
    </footer>
  </div>
</div>
