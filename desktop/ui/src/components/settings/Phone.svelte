<script lang="ts">
  import { SettingsRow, SettingsSection, Switch } from "@facile/muse";
  import { settings } from "../../lib/settings.svelte";
</script>

<SettingsSection title="Phone" description="Read-only view of boards, checks and the BOM on your phone.">
  <SettingsRow label="Allow pairing a phone on this network" description="Pairing needs a code shown by the studio and only works on your local network.">
    <Switch checked={settings.value?.allow_phone_pairing} aria-label="Allow pairing a phone on this network" onchange={(e) => settings.save({ allow_phone_pairing: e.currentTarget.checked })} />
  </SettingsRow>
  {#if settings.value?.allow_phone_pairing}
    <p class="pt-2 text-fc-sm text-fc-fg-muted">Open a project and press Pair phone in the studio top bar.</p>
  {/if}
</SettingsSection>
