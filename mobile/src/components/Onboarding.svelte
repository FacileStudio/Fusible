<script lang="ts">
  import { gsap } from "gsap";
  import { Button, Icon, prefersReducedMotion } from "@facile/muse";
  import { ICON } from "$lib/icons";
  import { useT, type Key } from "$lib/i18n";

  let { ondone }: { ondone: (scan: boolean) => void } = $props();
  const t = useT();

  const steps: { icon: string; size: number; title: Key; body: Key; action?: Key }[] = [
    { icon: ICON.phone, size: 56, title: "ob1Title", body: "ob1Body" },
    { icon: ICON.qr, size: 56, title: "ob2Title", body: "ob2Body" },
    { icon: ICON.scanner, size: 56, title: "ob3Title", body: "ob3Body", action: "ob3Action" },
  ];
  let index = $state(0);
  let panel: HTMLElement | undefined = $state();
  const step = $derived(steps[index]);
  const last = $derived(index === steps.length - 1);

  function slideIn() {
    if (!panel || prefersReducedMotion()) return;
    gsap.fromTo(panel, { x: 40, opacity: 0 }, { x: 0, opacity: 1, duration: 0.3, ease: "power2.out" });
  }

  function next() {
    if (last) return ondone(true);
    index += 1;
    queueMicrotask(slideIn);
  }
</script>

<section class="flex min-h-dvh flex-col justify-between gap-8 px-6 py-10">
  <div bind:this={panel} class="flex flex-1 flex-col items-center justify-center gap-5 text-center">
    <div class="rounded-fc-pill bg-fc-surface p-6 text-fc-fg">
      <Icon icon={step.icon} size={step.size} />
    </div>
    <h1 class="text-fc-2xl font-semibold">{t(step.title)}</h1>
    <p class="max-w-sm text-fc-md text-fc-fg-muted">{t(step.body)}</p>
  </div>
  <div class="flex flex-col items-stretch gap-2">
    <Button size="lg" onclick={next}>{step.action ? t(step.action) : t("continue")}</Button>
    <Button size="lg" variant="ghost" onclick={() => ondone(false)}>{t("skip")}</Button>
  </div>
  <p class="sr-only" aria-live="polite">{index + 1} / {steps.length}</p>
</section>
