<script lang="ts">
  import { Alert, Button, Progress, SettingsRow, SettingsSection, Switch } from "@facile/muse";
  import { api } from "../../lib/api";
  import { settings } from "../../lib/settings.svelte";
  import { checkedLabel, releaseUrl } from "../../lib/updateText";
  import { updater } from "../../lib/updater.svelte";

  let version = $state("");
  $effect(() => {
    void api.appVersion().then((v) => (version = v)).catch(() => undefined);
  });

  const busy = $derived(updater.state === "checking" || updater.state === "downloading");
  const status = $derived.by(() => {
    if (updater.state === "checking") return "Checking for updates";
    if (updater.state === "upToDate") return "Fusible Studio is up to date";
    if (updater.state === "available") return `Version ${updater.version} is available`;
    if (updater.state === "downloading") return `Downloading version ${updater.version}${updater.percent === null ? "" : `, ${updater.percent}%`}`;
    if (updater.state === "ready") return `Version ${updater.version} is ready. Restart to finish updating.`;
    return "";
  });
</script>

<SettingsSection title="Updates" description={version ? `Fusible Studio ${version}. ${checkedLabel(settings.value?.last_update_check ?? null)}.` : ""}>
  <SettingsRow label="Update automatically" description="Check for a new version at launch, once a day.">
    <Switch checked={settings.value?.auto_update} aria-label="Update automatically" onchange={(e) => settings.save({ auto_update: e.currentTarget.checked })} />
  </SettingsRow>
  <SettingsRow label={status || "Check for updates"} stacked>
    <div class="flex flex-col gap-3">
    {#if updater.state === "downloading"}
      <Progress value={updater.percent} label="Download progress" />
    {/if}
    {#if updater.state === "error"}
      <Alert tone="danger">
        {updater.error}
        {#if updater.errorUrl}
          <Button variant="ghost" size="sm" onclick={() => api.openUrl(updater.errorUrl!)}>Open the release page</Button>
        {:else if updater.version}
          <Button variant="ghost" size="sm" onclick={updater.installNow}>Try again</Button>
        {/if}
      </Alert>
    {/if}
    {#if updater.notes && (updater.state === "available" || updater.state === "ready")}
      <pre class="whitespace-pre-wrap rounded-fc-lg bg-fc-surface p-3 text-fc-xs">{updater.notes}</pre>
    {/if}
    <div class="flex flex-wrap gap-2">
      <Button variant="secondary" icon="solar:refresh-linear" disabled={busy} onclick={() => updater.checkNow(false)}>Check now</Button>
      {#if updater.state === "available"}
        <Button icon="solar:download-minimalistic-linear" onclick={updater.installNow}>Download and install</Button>
      {/if}
      {#if updater.state === "ready"}
        <Button icon="solar:restart-linear" onclick={updater.restartNow}>Restart to update</Button>
      {/if}
      {#if updater.version}
        <Button variant="ghost" onclick={() => api.openUrl(releaseUrl(updater.version!))}>Release notes</Button>
      {/if}
    </div>
    </div>
  </SettingsRow>
</SettingsSection>
