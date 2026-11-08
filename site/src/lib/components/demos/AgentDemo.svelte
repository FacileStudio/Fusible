<script lang="ts">
  import { fade } from "svelte/transition";
  import { Bar, Stage, Stream, Window, grow, type Act } from "@facile/mise-en-scene";
  import { Icon } from "@facile/muse";
  import type { AgentDemoCopy } from "$lib/content/types";

  let { copy }: { copy: AgentDemoCopy } = $props();

  type State = "hidden" | "running" | "ok" | "bad";
  const STEPS = [
    { name: "fusible_check", args: "main", end: "bad" },
    { name: "edit_board", args: "hardware/index.tsx", end: "ok" },
    { name: "fusible_check", args: "main", end: "ok" },
  ] as const;

  const words = $derived(copy.answer.split(" ").length);

  let draft = $state("");
  let sent = $state(false);
  let steps = $state<State[]>(["hidden", "hidden", "hidden"]);
  let streamed = $state(0);

  function reset() {
    draft = "";
    sent = false;
    steps = ["hidden", "hidden", "hidden"];
    streamed = 0;
  }

  function poster() {
    sent = true;
    steps = STEPS.map((step) => step.end);
    streamed = words;
  }

  async function script(act: Act) {
    await act.wait(700);
    await act.type(copy.prompt, (v) => (draft = v), 28);
    await act.wait(350);
    sent = true;
    draft = "";
    for (const [i, step] of STEPS.entries()) {
      await act.wait(350);
      steps[i] = "running";
      await act.wait(950);
      steps[i] = step.end;
    }
    await act.wait(400);
    await act.count(words, (v) => (streamed = v));
    await act.wait(2300);
    reset();
    await act.wait(600);
  }
</script>

<Stage width={560} height={430} compact={{ below: 480, width: 358, height: 460 }} {script} {reset} {poster} label={copy.prompt}>
  {#snippet children({ compact })}
    <Window style={compact ? "top:16px;left:16px;width:326px;height:428px" : "top:20px;left:28px;width:504px;height:390px"}>
      <div class="ad-thread">
        <div class="ad-past">
          <span class="ad-bubble"></span>
          <Bar w="86%" />
          <Bar w="62%" />
        </div>
        {#if sent}
          <div class="ad-q" in:grow out:fade={{ duration: 200, delay: 180 }}><p>{copy.prompt}</p></div>
        {/if}
        {#if steps[0] !== "hidden"}
          <ul class="ad-steps" in:grow out:fade={{ duration: 200, delay: 120 }}>
            {#each STEPS as step, i (i)}
              {#if steps[i] !== "hidden"}
                <li in:grow={{ duration: 300 }} data-state={steps[i]}>
                  <span class="ad-mark"></span><b>{step.name}</b><code>{step.args}</code>
                  {#if steps[i] === "bad"}<em>{copy.blown}</em>{:else if i === 2 && steps[i] === "ok"}<em>{copy.intact}</em>{/if}
                </li>
              {/if}
            {/each}
          </ul>
        {/if}
        {#if streamed > 0}
          <div in:grow={{ duration: 300 }} out:fade={{ duration: 200 }}>
            <Stream class="ad-answer" text={copy.answer} count={streamed} />
          </div>
        {/if}
      </div>
      <div class="ad-composer">
        <Icon icon="solar:pen-new-square-linear" size={15} class="ad-lead-icon" />
        <span class="ad-input" class:ad-placeholder={!draft}>{draft || copy.placeholder}</span>
        <span class="ad-send" class:ad-idle={!draft}>
          <Icon icon="solar:arrow-up-linear" size={16} />
        </span>
      </div>
    </Window>
  {/snippet}
</Stage>

<style>
  .ad-thread {
    display: flex;
    flex: 1;
    flex-direction: column;
    justify-content: flex-end;
    min-height: 0;
    overflow: hidden;
    padding: 0 16px 12px;
    font-size: 13px;
    color: var(--color-fc-fg);
    mask-image: linear-gradient(to bottom, transparent, #000 40px);
  }
  .ad-past {
    display: grid;
    gap: 8px;
    padding-bottom: 8px;
  }
  .ad-bubble {
    justify-self: end;
    width: 40%;
    height: 32px;
    border-radius: 12px;
    background: var(--color-fc-surface);
  }
  .ad-q {
    display: flex;
    justify-content: flex-end;
    padding-top: 12px;
  }
  .ad-q p {
    max-width: 85%;
    margin: 0;
    padding: 8px 14px;
    border-radius: 12px;
    background: var(--color-fc-surface);
  }
  .ad-steps {
    margin: 12px 0 0;
    padding: 4px 0;
    list-style: none;
    border-radius: 12px;
    background: var(--color-fc-component);
  }
  .ad-steps li {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 7px 12px;
    white-space: nowrap;
  }
  .ad-steps b {
    font-weight: 500;
    font-size: 12px;
  }
  .ad-steps code {
    overflow: hidden;
    text-overflow: ellipsis;
    font-family: var(--font-fc-mono);
    font-size: 11px;
    color: var(--color-fc-fg-muted);
  }
  .ad-steps em {
    margin-left: auto;
    font-style: normal;
    font-size: 11px;
    color: var(--color-fc-success);
  }
  [data-state="bad"] em {
    color: var(--color-fc-danger);
  }
  .ad-mark {
    flex: none;
    width: 8px;
    height: 8px;
    border-radius: 999px;
    background: var(--color-fc-fg-muted);
    transition: background 0.2s ease;
  }
  [data-state="ok"] .ad-mark {
    background: var(--color-fc-success);
  }
  [data-state="bad"] .ad-mark {
    background: var(--color-fc-danger);
  }
  :global(.ad-answer) {
    margin: 0;
    padding-top: 10px;
    line-height: 1.55;
  }
  .ad-composer {
    display: flex;
    align-items: center;
    gap: 8px;
    margin: 0 16px 16px;
    padding: 6px 6px 6px 14px;
    border-radius: 999px;
    background: var(--color-fc-field);
    font-size: 13px;
    color: var(--color-fc-fg);
  }
  :global(.ad-lead-icon) {
    flex: none;
    color: var(--color-fc-fg-muted);
  }
  .ad-input {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }
  .ad-placeholder {
    color: var(--color-fc-fg-muted);
  }
  .ad-send {
    display: flex;
    align-items: center;
    justify-content: center;
    flex: none;
    margin-left: auto;
    width: 32px;
    height: 32px;
    border-radius: 999px;
    background: var(--color-fc-accent);
    color: var(--color-fc-accent-fg);
    transition: opacity 0.15s ease;
  }
  .ad-idle {
    opacity: 0.4;
  }
</style>
