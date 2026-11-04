<script lang="ts">
  import { Tabs } from "@facile/muse";
  import { setView, studio } from "../lib/studio.svelte";
  import type { ViewName } from "../lib/types";

  const ALL = [
    { id: "schematic", label: "Schematic", icon: "solar:routing-2-linear" },
    { id: "pcb", label: "PCB", icon: "solar:cpu-linear" },
    { id: "3d", label: "3D", icon: "solar:box-linear" },
    { id: "mechanical", label: "Bodies", icon: "solar:box-minimalistic-linear" },
    { id: "assembly", label: "Assembly", icon: "solar:layers-linear" },
    { id: "bom", label: "Parts", icon: "solar:bill-list-linear" },
  ];

  const items = $derived(ALL.filter((tab) => tab.id !== "assembly" || studio.hasAssembly));
</script>

<Tabs
  {items}
  value={studio.view}
  label="Views"
  class="min-w-0 border-0"
  onChange={(id) => setView(id as ViewName)}
/>
