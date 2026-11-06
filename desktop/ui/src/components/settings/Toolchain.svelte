<script lang="ts">
  import { Alert, Badge, Icon, SettingsRow, SettingsSection } from "@facile/muse";
  import { api, message } from "../../lib/api";
  import type { ToolchainStatus } from "../../lib/types";
  import CodeBlock from "../CodeBlock.svelte";

  let status = $state<ToolchainStatus | null>(null);
  let error = $state<string | null>(null);

  $effect(() => {
    api.toolchainStatus().then((s) => (status = s)).catch((e) => (error = message(e)));
  });

  const rows = $derived(status ? [
    { name: "bun", tool: status.bun },
    { name: "node", tool: status.node },
    { name: "tsci", tool: status.tsci },
  ] : []);
  const missing = $derived(rows.some((r) => !r.tool));
</script>

<SettingsSection title="Toolchain" description="Tools Fusible calls to compile boards and firmware.">
  {#if error}
    <Alert tone="danger">{error}</Alert>
  {:else if !status}
    <span role="status" aria-label="Loading" class="inline-flex"><Icon icon="solar:refresh-linear" size={24} class="motion-safe:animate-spin" /></span>
  {:else}
    {#each rows as row (row.name)}
      <SettingsRow label={row.name} description={row.tool?.path ?? "Not found"}>
        <Badge tone={row.tool ? "success" : "danger"}>
          <Icon icon={row.tool ? "solar:check-circle-linear" : "solar:close-circle-linear"} size={14} />
          {row.tool?.version ?? "Missing"}
        </Badge>
      </SettingsRow>
    {/each}
    {#if status.bundled_bun}
      <p class="pt-2 text-fc-sm text-fc-fg-muted">Fusible bundles bun {status.bundled_bun}.</p>
    {/if}
    {#if missing}
      <div class="pt-4"><CodeBlock title="Install the missing tools" code="fusible doctor --install" /></div>
    {/if}
  {/if}
</SettingsSection>
