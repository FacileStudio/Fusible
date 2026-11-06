<script lang="ts">
  import { EmptyState, Icon } from "@facile/muse";
  import { studio } from "../../lib/studio.svelte";
</script>

{#if studio.diagnostics.length}
  <div class="flex flex-col gap-2">
    {#each studio.diagnostics as diag, index (index)}
      <div class="flex flex-col gap-2 rounded-2xl bg-fc-component px-4 py-3 text-fc-sm select-text">
        <div class="flex items-center gap-2 font-medium text-fc-danger">
          <Icon icon="solar:danger-triangle-bold-duotone" size={16} />
          <span>{diag.rule || "violation"}</span>
        </div>
        <p>{diag.message || ""}</p>
        {#if diag.help}<p class="text-fc-fg-muted">{diag.help}</p>{/if}
        {#if diag.fix}<p class="text-fc-success">Fix: {diag.fix}</p>{/if}
        {#if diag.standard}
          <div class="flex flex-wrap items-center gap-2 text-fc-xs">
            {#if diag.source_url}
              <a
                class="inline-flex h-7 items-center rounded-full bg-fc-surface px-3 hover:text-fc-info focus-visible:outline-2 focus-visible:outline-fc-ring"
                href={diag.source_url}
                target="_blank"
                rel="noopener noreferrer"
              >
                {diag.standard}
              </a>
            {:else}
              <span class="inline-flex h-7 items-center rounded-full bg-fc-surface px-3">{diag.standard}</span>
            {/if}
            <span class="text-fc-fg-muted">Design pre-check, not certification.</span>
          </div>
        {/if}
      </div>
    {/each}
  </div>
{:else}
  <EmptyState bare title="No electrical violations detected" />
{/if}
