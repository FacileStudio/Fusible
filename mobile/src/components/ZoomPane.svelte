<script lang="ts">
  import type { Snippet } from "svelte";

  let { children }: { children: Snippet } = $props();

  let scale = $state(1);
  let x = $state(0);
  let y = $state(0);
  const pointers = new Map<number, { x: number; y: number }>();
  let pinch = 0;

  const spread = () => {
    const [a, b] = [...pointers.values()];
    return Math.hypot(a.x - b.x, a.y - b.y);
  };

  function down(e: PointerEvent) {
    (e.currentTarget as HTMLElement).setPointerCapture(e.pointerId);
    pointers.set(e.pointerId, { x: e.clientX, y: e.clientY });
    if (pointers.size === 2) pinch = spread();
  }

  function move(e: PointerEvent) {
    const last = pointers.get(e.pointerId);
    if (!last) return;
    pointers.set(e.pointerId, { x: e.clientX, y: e.clientY });
    if (pointers.size === 2) {
      const now = spread();
      scale = Math.min(8, Math.max(1, scale * (now / pinch)));
      pinch = now;
    } else {
      x += e.clientX - last.x;
      y += e.clientY - last.y;
    }
  }

  function up(e: PointerEvent) {
    pointers.delete(e.pointerId);
    if (scale === 1) [x, y] = [0, 0];
  }

  function reset() {
    [scale, x, y] = [1, 0, 0];
  }
</script>

<div
  class="aspect-[4/3] max-h-[60dvh] touch-none overflow-hidden rounded-fc-lg bg-fc-surface"
  role="img"
  aria-label="Board view"
  onpointerdown={down}
  onpointermove={move}
  onpointerup={up}
  onpointercancel={up}
  ondblclick={reset}
>
  <div class="flex h-full w-full origin-center items-center justify-center" style:transform={`translate(${x}px, ${y}px) scale(${scale})`}>
    {@render children()}
  </div>
</div>
