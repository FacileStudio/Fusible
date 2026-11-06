<script lang="ts" generics="T extends string">
  type Props = { options: { value: T; label: string }[]; value: T; label: string; onChange: (value: T) => void };
  let { options, value, label, onChange }: Props = $props();
</script>

<div role="group" aria-label={label} class="inline-flex gap-1 rounded-full bg-fc-surface p-1">
  {#each options as option (option.value)}
    <button
      type="button"
      aria-pressed={option.value === value}
      class="rounded-full px-4 py-1.5 text-fc-sm transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-fc-ring {option.value === value ? 'bg-fc-accent text-fc-accent-fg' : 'text-fc-fg-muted hover:text-fc-fg'}"
      onclick={() => onChange(option.value)}
    >
      {option.label}
    </button>
  {/each}
</div>
