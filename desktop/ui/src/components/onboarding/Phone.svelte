<script lang="ts">
  import { Icon, Switch } from "@facile/muse";
  import { settings } from "../../lib/settings.svelte";
</script>

<Icon icon="solar:smartphone-bold-duotone" size={56} />
<div class="flex flex-col gap-2">
  <h1 class="text-fc-2xl font-semibold">Check your boards from your phone</h1>
  <p class="text-fc-md text-fc-fg-muted">
    Your phone gets a read-only view of boards, checks and the BOM. Pairing needs a code shown by the studio and
    only works on your local network.
  </p>
</div>
<Switch
  checked={settings.value?.allow_phone_pairing}
  label="Allow pairing a phone on this network"
  onchange={(e) => settings.save({ allow_phone_pairing: e.currentTarget.checked })}
/>
