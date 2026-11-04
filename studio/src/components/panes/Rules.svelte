<script lang="ts">
  import { EmptyState } from "@facile/muse";
  import { catalog } from "../../lib/catalog.svelte";
</script>

{#if catalog.rules.length}
  <div class="flex flex-col gap-2">
    {#each catalog.rules as rule (rule.id)}
      <div class="flex flex-col gap-1 rounded-2xl bg-fc-component px-4 py-3 text-fc-sm select-text">
        <div class="flex items-center justify-between gap-3">
          <span class="font-medium">{rule.id}</span>
          <span class="shrink-0 rounded-full bg-fc-surface px-3 py-0.5 text-fc-xs {rule.enabled ? 'text-fc-success' : 'text-fc-fg-muted'}">
            {rule.enabled ? "On" : "Off"}
          </span>
        </div>
        <p class="text-fc-fg-muted">{rule.summary}</p>
        <div class="flex flex-wrap items-center gap-2 text-fc-xs">
          <a
            class="inline-flex h-7 items-center rounded-full bg-fc-surface px-3 hover:text-fc-info focus-visible:outline-2 focus-visible:outline-fc-ring"
            href={rule.source_url}
            target="_blank"
            rel="noopener noreferrer"
          >
            {rule.standard}
          </a>
          <span class="text-fc-fg-muted">{rule.severity}. Design pre-check, not certification.</span>
        </div>
      </div>
    {/each}
  </div>
{:else}
  <EmptyState bare title="No rules reported" />
{/if}
