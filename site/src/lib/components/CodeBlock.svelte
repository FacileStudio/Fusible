<script lang="ts">
  import type { Content } from "$lib/content/types";
  import CopyButton from "./CopyButton.svelte";

  interface Token {
    text: string;
    cls?: string;
  }

  let { code, content, label }: { code: string; content: Content; label?: string } = $props();

  function tokenizeLine(line: string): Token[] {
    const tokens: Token[] = [];
    const commentIdx = line.indexOf("#");
    let codePart = line;
    let commentPart = "";
    if (commentIdx !== -1) {
      codePart = line.slice(0, commentIdx);
      commentPart = line.slice(commentIdx);
    }

    const sectionMatch = codePart.match(/^(\s*)(\[.+\])(\s*)$/);
    if (sectionMatch) {
      if (sectionMatch[1]) tokens.push({ text: sectionMatch[1] });
      tokens.push({ text: sectionMatch[2], cls: "text-fc-accent font-medium" });
      if (sectionMatch[3]) tokens.push({ text: sectionMatch[3] });
    } else {
      const kvMatch = codePart.match(/^(\s*)([a-zA-Z0-9_.-]+)(\s*[:=]\s*)(.*)$/);
      if (kvMatch) {
        if (kvMatch[1]) tokens.push({ text: kvMatch[1] });
        tokens.push({ text: kvMatch[2], cls: "text-fc-info font-medium" });
        tokens.push({ text: kvMatch[3], cls: "text-fc-fg-muted" });
        const val = kvMatch[4];
        if (val) {
          const listMatch = val.match(/^(\[)(.*)(\])$/);
          if (listMatch) {
            tokens.push({ text: "[", cls: "text-fc-fg-muted" });
            const inner = listMatch[2].split(/(\s*,\s*)/);
            for (const part of inner) {
              if (part.includes(",")) {
                tokens.push({ text: part, cls: "text-fc-fg-muted" });
              } else if (part.trim()) {
                tokens.push({ text: part, cls: "text-fc-success" });
              } else {
                tokens.push({ text: part });
              }
            }
            tokens.push({ text: "]", cls: "text-fc-fg-muted" });
          } else if (val.trim()) {
            tokens.push({ text: val, cls: "text-fc-success" });
          } else {
            tokens.push({ text: val });
          }
        }
      } else {
        const words = codePart.split(/(\s+)/);
        let first = true;
        for (const w of words) {
          if (!w) continue;
          if (/^\s+$/.test(w)) {
            tokens.push({ text: w });
          } else if (first) {
            tokens.push({ text: w, cls: "text-fc-info font-medium" });
            first = false;
          } else if (w.startsWith("-")) {
            tokens.push({ text: w, cls: "text-fc-fg-muted" });
          } else {
            tokens.push({ text: w, cls: "text-fc-fg" });
          }
        }
      }
    }

    if (commentPart) {
      tokens.push({ text: commentPart, cls: "text-fc-fg-muted italic" });
    }
    return tokens;
  }

  const tokenizedLines = $derived(code.split("\n").map(tokenizeLine));
</script>

<div class="relative rounded-fc-md bg-fc-surface">
  {#if label}
    <p class="px-4 pt-3 text-fc-xs text-fc-fg-muted">{label}</p>
  {/if}
  <div
    class="overflow-x-auto p-4 pr-14"
    tabindex="0"
    role="textbox"
    aria-readonly="true"
    aria-multiline="true"
    aria-label={label ?? content.ui.code}
  >
    <pre class="font-fc-mono text-fc-xs leading-relaxed text-fc-fg"><code>{#each tokenizedLines as line, i (i)}{#each line as token}{#if token.cls}<span class={token.cls}>{token.text}</span>{:else}{token.text}{/if}{/each}{#if i < tokenizedLines.length - 1}{"\n"}{/if}{/each}</code></pre>
  </div>
  <div class="absolute right-2 top-2"><CopyButton text={code} {content} /></div>
</div>
