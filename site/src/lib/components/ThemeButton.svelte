<script lang="ts">
  import { browser } from "$app/environment";
  import { Icon, IconButton, icons } from "@facile/muse";
  import type { Content } from "$lib/content/types";

  let { content }: { content: Content } = $props();

  let theme = $state<"light" | "dark">(
    browser && document.documentElement.classList.contains("light") ? "light" : "dark",
  );

  function toggle() {
    theme = theme === "dark" ? "light" : "dark";
    document.documentElement.classList.toggle("dark", theme === "dark");
    document.documentElement.classList.toggle("light", theme === "light");
    try {
      localStorage.setItem("fusible-theme", theme);
    } catch {}
  }
</script>

<IconButton
  variant="ghost"
  class="size-9 pointer-coarse:size-11 bg-fc-surface/70 text-fc-fg hover:bg-fc-surface"
  aria-label={theme === "dark" ? content.ui.themeToLight : content.ui.themeToDark}
  onclick={toggle}
>
  <Icon icon={theme === "dark" ? icons.sun : icons.moon} size={16} />
</IconButton>
