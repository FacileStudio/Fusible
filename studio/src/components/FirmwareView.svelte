<script lang="ts">
  import { Alert, Button, EmptyState, Icon } from "@facile/muse";
  import { launch } from "../lib/dispatch.svelte";
  import { firmware, lastBuild, selectPin } from "../lib/firmware.svelte";
  import { isBusy } from "../lib/runs.svelte";

  const data = $derived(firmware.data);
  const build = $derived(firmware.name ? lastBuild(firmware.name) : undefined);
  const STATUS = { running: "Running", ok: "OK", fuse_blown: "Fuse blown", error: "Failed" };
  const meta = $derived(
    data ? [data.board && `Board ${data.board}`, `MCU ${data.mcu}`, data.framework && `Framework ${data.framework}`] : [],
  );
  const groups = $derived.by(() => {
    const out: Record<string, NonNullable<typeof data>["pins"]> = {};
    data?.pins.forEach((pin) => (out[pin.group] = [...(out[pin.group] || []), pin]));
    return out;
  });
</script>

<div class="flex flex-col gap-2 text-fc-sm">
  <div class="flex flex-col gap-2 rounded-fc-lg bg-fc-surface px-4 py-3">
    <div class="font-medium">{firmware.name}</div>
    <div class="text-fc-xs text-fc-fg-muted">{meta.filter(Boolean).join(" · ")}</div>
    <div class="flex items-center gap-3">
      {#if !data || data.buildable}
        <Button size="sm" icon="solar:play-linear" disabled={isBusy()} onclick={() => firmware.name && launch("build", firmware.name)}>
          Build
        </Button>
      {/if}
      <span class="text-fc-xs text-fc-fg-muted">{build ? STATUS[build.status] || build.status : "Not built"}</span>
    </div>
  </div>
  {#if firmware.error}
    <Alert tone="danger">{firmware.error}</Alert>
  {:else if !data}
    <EmptyState bare title="Loading pin map..." />
  {:else}
    {#if data.pins_rs}
      <div class="px-2 text-fc-xs text-fc-fg-muted">{data.pins_rs} · {data.generated ? "Generated" : "Not generated yet"}</div>
    {/if}
    {#if data.violations.length}
      <Alert tone="danger">{data.violations.length} pin-mux conflict{data.violations.length === 1 ? "" : "s"}</Alert>
    {/if}
    {#each Object.entries(groups) as [group, pins] (group)}
      <div class="px-2 pt-2 text-fc-xs text-fc-fg-muted">{group} · {pins.length}</div>
      {#each pins as pin (pin.pin)}
        {@const part = pin.remote ? pin.remote.split(".")[0] : ""}
        <button
          type="button"
          class="flex flex-col gap-0.5 rounded-fc-lg px-3 py-2 text-left hover:bg-fc-surface focus-visible:outline-2 focus-visible:outline-fc-ring {firmware.picked === pin.pin
            ? 'bg-fc-surface'
            : ''}"
          title={pin.net}
          onclick={() => selectPin(pin.pin, pin.net, part)}
        >
          <span class="flex items-center justify-between gap-3">
            <span class="font-medium {pin.violation ? 'text-fc-danger' : ''}">{pin.pin}</span>
            <span class="text-fc-xs text-fc-fg-muted">{pin.const.replace(/^gpio::/, "")}</span>
          </span>
          <span class="flex items-center justify-between gap-3 text-fc-xs text-fc-fg-muted">
            <span>{pin.remote || ""}</span>
            <span class="font-fc-mono">{pin.field}</span>
          </span>
          {#if pin.violation}
            <span class="flex items-center gap-1 text-fc-xs text-fc-danger">
              <Icon icon="solar:danger-triangle-bold-duotone" size={14} />{pin.violation}
            </span>
          {/if}
        </button>
      {/each}
    {:else}
      <EmptyState bare title="No GPIO pins mapped" />
    {/each}
  {/if}
</div>
