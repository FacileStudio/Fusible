<script lang="ts">
  import { Badge, Icon } from "@facile/muse";
  import { ICON } from "$lib/icons";
  import { useT } from "$lib/i18n";

  let { blown }: { blown: boolean } = $props();
  const t = useT();
</script>

<Badge tone={blown ? "danger" : "success"}>
  <span class="inline-flex items-center gap-1.5">
    <Icon icon={blown ? ICON.warn : ICON.ok} size={14} />
    {blown ? t("fuseBlown") : t("fuseIntact")}
  </span>
</Badge>
