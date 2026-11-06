<script lang="ts">
  import { Icon } from "@facile/muse";
  import { api } from "../../lib/api";
  import type { ToolchainStatus } from "../../lib/types";
  import CodeBlock from "../CodeBlock.svelte";

  let status = $state<ToolchainStatus | null>(null);

  $effect(() => {
    api.toolchainStatus().then((s) => (status = s)).catch(() => (status = { bun: null, node: null, tsci: null }));
  });

  const rows = $derived(status ? [
    { name: "bun", tool: status.bun },
    { name: "node", tool: status.node },
    { name: "tsci", tool: status.tsci },
  ] : []);
</script>

<div class="flex flex-col gap-2">
  <h1 class="text-fc-2xl font-semibold">Your toolchain</h1>
  <p class="text-fc-md text-fc-fg-muted">Fusible compiles boards and firmware with these tools.</p>
</div>
{#if !status}
  <span role="status" aria-label="Loading" class="inline-flex"><Icon icon="solar:refresh-linear" size={24} class="motion-safe:animate-spin" /></span>
{:else}
  <ul class="flex flex-col gap-2">
    {#each rows as row (row.name)}
      <li class="flex items-center gap-3 rounded-2xl bg-fc-surface px-4 py-3">
        <Icon
          icon={row.tool ? "solar:check-circle-linear" : "solar:close-circle-linear"}
          size={20}
          class={row.tool ? "text-fc-success" : "text-fc-danger"}
        />
        <span class="flex-1 text-fc-sm font-medium">{row.name}</span>
        <span class="text-fc-xs text-fc-fg-muted">{row.tool?.version ?? "Not found"}</span>
      </li>
    {/each}
  </ul>
  {#if status.bundled_bun}
    <p class="text-fc-sm">Fusible bundles bun, you are ready.</p>
  {:else}
    <CodeBlock title="Install the missing tools" code="fusible doctor --install" />
  {/if}
{/if}
