<script lang="ts">
  import { LABEL, WINDOW_BUTTONS, invoke } from "../lib/desktop";

  const TONES = { close: "bg-fc-danger", min: "bg-fc-warning", max: "bg-fc-success" };
</script>

<div class="group flex shrink-0 items-center gap-2 px-2" role="group" aria-label="Window controls" data-tauri-drag-region>
  {#each WINDOW_BUTTONS as button (button.name)}
    <button
      type="button"
      class="grid size-3 place-items-center rounded-full text-[11px] leading-none text-fc-page {TONES[button.name]} focus-visible:outline-2 focus-visible:outline-fc-ring"
      title={button.label}
      aria-label={button.label}
      onclick={() => invoke(button.cmd, { label: LABEL })}
    >
      <span class="opacity-0 group-hover:opacity-100" aria-hidden="true">{button.glyph}</span>
    </button>
  {/each}
</div>
