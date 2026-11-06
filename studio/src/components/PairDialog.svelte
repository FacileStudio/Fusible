<script lang="ts">
  import { Modal } from "@facile/muse";
  import QRCode from "qrcode";
  import { ApiError, api } from "../lib/api";
  import { remaining } from "../lib/pair";
  import { studio } from "../lib/studio.svelte";
  import type { PairStart } from "../lib/types";

  let pair = $state<PairStart | null>(null);
  let svg = $state("");
  let missing = $state(false);
  let error = $state("");
  let now = $state(Date.now());

  const left = $derived(pair ? remaining(pair.expires_at, now) : 0);

  async function start() {
    pair = null;
    svg = "";
    missing = false;
    error = "";
    try {
      pair = await api.startPair();
      svg = await QRCode.toString(pair.uri, { type: "svg", margin: 0 });
    } catch (err) {
      missing = err instanceof ApiError && err.status === 404;
      error = missing ? "" : String((err as Error).message || err);
    }
  }

  $effect(() => {
    if (!studio.pairOpen) return;
    void start();
    const tick = setInterval(() => (now = Date.now()), 1000);
    return () => clearInterval(tick);
  });
</script>

<Modal bind:open={studio.pairOpen} title="Pair a phone" showClose size="sm">
  <div class="flex flex-col items-center gap-4 p-6 text-fc-sm">
    {#if missing}
      <p>Run fusible studio --share to pair a phone.</p>
    {:else if error}
      <p class="text-fc-danger" role="alert">{error}</p>
    {:else if pair}
      <div class="size-56 rounded-3xl bg-white p-4" aria-label="Pairing code">{@html svg}</div>
      <p class="text-fc-fg-muted">
        {left > 0 ? `Scan with the Fusible app. Expires in ${Math.floor(left / 60)}:${String(left % 60).padStart(2, "0")}.` : "This code expired."}
      </p>
      {#if left <= 0}
        <button type="button" class="h-9 rounded-full bg-fc-accent px-4 text-fc-accent-fg" onclick={start}>New code</button>
      {/if}
    {:else}
      <p class="text-fc-fg-muted">Creating a code...</p>
    {/if}
  </div>
</Modal>
