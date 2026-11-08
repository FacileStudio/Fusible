<script lang="ts">
  import { Field, Input, RadioGroup, Select, SettingsRow, SettingsSection, Switch } from "@facile/muse";
  import { currencyChoices } from "../../lib/labels";
  import { settings } from "../../lib/settings.svelte";
  import type { Theme } from "../../lib/types";
  import { validatePort } from "../../lib/validate";
  import PathField from "../PathField.svelte";

  const s = $derived(settings.value!);
  let dir = $state(settings.value?.projects_dir ?? "");
  let port = $state(settings.value?.port ?? 3020);
  const portError = $derived(validatePort(port));
  const themes: { value: Theme; label: string }[] = [
    { value: "system", label: "System" },
    { value: "dark", label: "Dark" },
    { value: "light", label: "Light" },
  ];
</script>

<SettingsSection title="General">
  <SettingsRow label="Projects folder" description="Where new projects are created." stacked>
    <PathField bind:value={dir} title="Choose the projects folder" label="Projects folder" onPick={(p) => settings.save({ projects_dir: p })} />
  </SettingsRow>
  <SettingsRow label="Default currency" description="Used for the BOM of new projects.">
    <Select value={s.default_currency} class="w-32" onchange={(e) => settings.save({ default_currency: e.currentTarget.value })}>
      {#each currencyChoices(s.default_currency) as code (code)}<option value={code}>{code}</option>{/each}
    </Select>
  </SettingsRow>
  <SettingsRow label="Studio port" description="The local port the studio listens on.">
    <Field error={portError ?? undefined}>
      {#snippet children()}
        <Input type="number" bind:value={port} min={1024} max={65535} class="w-32" onchange={() => { if (!portError) void settings.save({ port }); }} />
      {/snippet}
    </Field>
  </SettingsRow>
  <SettingsRow label="Open the last project on launch">
    <Switch checked={s.open_last_project} aria-label="Open the last project on launch" onchange={(e) => settings.save({ open_last_project: e.currentTarget.checked })} />
  </SettingsRow>
  <SettingsRow label="Theme">
    <RadioGroup
      options={themes}
      value={s.theme}
      name="theme"
      orientation="horizontal"
      aria-label="Theme"
      onchange={(e) => settings.save({ theme: (e.target as HTMLInputElement).value as Theme })}
    />
  </SettingsRow>
</SettingsSection>
