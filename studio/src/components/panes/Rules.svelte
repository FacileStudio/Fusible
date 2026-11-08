<script lang="ts">
  import { Badge, EmptyState } from "@facile/muse";
  import { catalog } from "../../lib/catalog.svelte";
</script>

{#if catalog.rules.length}
  <div class="flex flex-col gap-2">
    {#each catalog.rules as rule (rule.id)}
      <div class="flex flex-col gap-1 rounded-fc-lg bg-fc-surface px-4 py-3 text-fc-sm select-text">
        <div class="flex items-center justify-between gap-3">
          <span class="font-medium">{rule.id}</span>
          <Badge tone={rule.enabled ? "success" : "neutral"}>{rule.enabled ? "On" : "Off"}</Badge>
        </div>
        <p class="text-fc-fg-muted">{rule.summary}</p>
        {#if rule.standard}
          <a
            class="inline-flex h-7 items-center self-start rounded-fc-pill bg-fc-field px-3 text-fc-xs hover:text-fc-info focus-visible:outline-2 focus-visible:outline-fc-ring"
            href={rule.source_url}
            target="_blank"
            rel="noopener noreferrer"
          >
            {rule.standard}
          </a>
        {/if}
      </div>
    {/each}
    <p class="px-2 py-1 text-fc-xs text-fc-fg-muted">Design pre-checks, not certification.</p>
  </div>
{:else}
  <EmptyState bare title="No rules reported" />
{/if}
