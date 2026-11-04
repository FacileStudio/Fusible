<script lang="ts">
  import { Alert, Button, Field, Input, Modal, Select, Switch } from "@facile/muse";
  import { api, message } from "../lib/api";
  import { currencyChoices } from "../lib/labels";
  import { finalPath } from "../lib/path";
  import { projects } from "../lib/projects.svelte";
  import { settings } from "../lib/settings.svelte";
  import { validateName } from "../lib/validate";
  import PathField from "./PathField.svelte";

  let { open = $bindable(false) }: { open?: boolean } = $props();

  let name = $state("");
  let location = $state("");
  let createFolder = $state(true);
  let currency = $state("EUR");
  let touched = $state(false);
  let busy = $state(false);
  let failure = $state<string | null>(null);

  const nameError = $derived(touched ? validateName(name) : null);
  const valid = $derived(!validateName(name) && location.trim() !== "");
  const target = $derived(finalPath(location, name, createFolder));

  $effect(() => {
    if (!open) return;
    const fill = projects.prefill;
    name = fill?.name ?? "";
    location = fill?.location ?? settings.value?.projects_dir ?? "";
    createFolder = !fill;
    currency = settings.value?.default_currency ?? "EUR";
    touched = Boolean(fill);
    failure = null;
  });

  async function create(): Promise<void> {
    busy = true;
    failure = null;
    try {
      const made = await api.createProject({ parent: location.trim(), name: name.trim(), create_folder: createFolder, currency });
      open = false;
      projects.prefill = null;
      await projects.open(made.path, made.name);
    } catch (error) {
      failure = message(error);
    }
    busy = false;
  }
</script>

<Modal bind:open title="New project">
  <form class="flex flex-col gap-4" onsubmit={(e) => { e.preventDefault(); if (valid && !busy) void create(); }}>
    <Field label="Name" error={nameError ?? undefined}>
      {#snippet children()}
        <Input bind:value={name} placeholder="My board" maxlength={120} autocomplete="off" class="border-0 bg-fc-surface" oninput={() => (touched = true)} />
      {/snippet}
    </Field>
    <Field label="Location">
      {#snippet children()}
        <PathField bind:value={location} title="Choose a location" label="Location" />
      {/snippet}
    </Field>
    <Switch bind:checked={createFolder} label="Create a folder named after the project" />
    <Field label="Currency">
      {#snippet children()}
        <Select bind:value={currency} class="border-0 bg-fc-surface">
          {#each currencyChoices(currency) as code (code)}<option value={code}>{code}</option>{/each}
        </Select>
      {/snippet}
    </Field>
    <p class="text-fc-xs text-fc-fg-muted">Final path: <output class="break-all">{target || "Choose a location"}</output></p>
    {#if failure}<Alert tone="danger">{failure}</Alert>{/if}
    <div class="flex justify-end gap-2">
      <Button variant="ghost" onclick={() => (open = false)}>Cancel</Button>
      <Button type="submit" disabled={!valid || busy}>{busy ? "Creating" : "Create project"}</Button>
    </div>
  </form>
</Modal>
