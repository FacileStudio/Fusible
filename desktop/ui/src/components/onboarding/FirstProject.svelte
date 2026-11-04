<script lang="ts">
  import { OptionCards } from "@facile/muse";
  import NewProject from "../NewProject.svelte";
  import { projects } from "../../lib/projects.svelte";
  import { settings } from "../../lib/settings.svelte";

  let creating = $state(false);
  let choice = $state("");

  const options = [
    { value: "create", label: "Create a project", icon: "solar:add-folder-bold-duotone" },
    { value: "open", label: "Open a folder", icon: "solar:folder-open-bold-duotone" },
  ];

  async function pick(value: string): Promise<void> {
    if (value === "create") {
      projects.prefill = null;
      creating = true;
      return;
    }
    await settings.save({ onboarded: true });
    if (await projects.openFolder()) creating = true;
  }
</script>

<div class="flex flex-col gap-2">
  <h1 class="text-fc-2xl font-semibold">Your first project</h1>
  <p class="text-fc-md text-fc-fg-muted">Start fresh or open a folder that already has a fusible.yml. You can skip this.</p>
</div>
<OptionCards bind:value={choice} {options} label="First project" onSelect={pick} />
<NewProject bind:open={creating} />
