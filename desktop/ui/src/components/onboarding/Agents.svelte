<script lang="ts">
  import { Icon } from "@facile/muse";
  import CodeBlock from "../CodeBlock.svelte";

  const bulle = ["sources:", "  mcp:", "    fusible:", "      command: fusible", "      args: [mcp]"].join("\n");
</script>

<Icon icon="solar:chat-round-dots-bold-duotone" size={56} />
<div class="flex flex-col gap-2">
  <h1 class="text-fc-2xl font-semibold">Let agents work on your boards</h1>
  <p class="text-fc-md text-fc-fg-muted">
    Fusible speaks MCP. Bulle, Claude Code or Codex can check boards, edit them and build parts, and every edit
    returns the fuse verdict.
  </p>
</div>
<CodeBlock title="Bulle (~/.config/bulle/config.yml)" code={bulle} />
<CodeBlock title="Claude Code" code="claude mcp add fusible -- fusible mcp" />
