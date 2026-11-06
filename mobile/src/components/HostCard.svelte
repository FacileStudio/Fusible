<script lang="ts">
  import { Icon, IconButton } from "@facile/muse";
  import type { Host } from "$lib/hosts";
  import { ICON } from "$lib/icons";
  import { useT } from "$lib/i18n";

  let { host, onforget }: { host: Host; onforget: (id: string) => void } = $props();
  const t = useT();
</script>

<div class="relative flex items-center gap-3 rounded-3xl bg-fc-surface p-5">
  <Icon icon={ICON.logo} size={32} class="shrink-0 text-fc-fg-muted" />
  <a
    href={`/host?id=${encodeURIComponent(host.id)}`}
    class="min-w-0 flex-1 rounded-full focus-visible:outline-2 focus-visible:outline-fc-ring"
  >
    <span class="block truncate text-fc-md font-semibold">{host.name}</span>
    <span class="block truncate text-fc-xs text-fc-fg-muted">
      {host.unpaired ? t("unpaired") : host.url.replace("http://", "")}
    </span>
  </a>
  <IconButton variant="ghost" aria-label={t("forget")} onclick={() => onforget(host.id)}>
    <Icon icon={ICON.forget} size={18} />
  </IconButton>
</div>
