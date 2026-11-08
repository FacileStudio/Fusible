<script lang="ts">
  import { Icon } from "@facile/muse";

  const features = [
    { icon: "solar:bolt-circle-bold-duotone", text: "Checks electrical and physical rules" },
    { icon: "solar:code-square-bold-duotone", text: "Generates firmware pin bindings" },
    { icon: "solar:box-bold-duotone", text: "Builds parts and enclosures from code" },
  ];
</script>

<Icon icon="solar:cpu-bolt-bold-duotone" size={56} />
<div class="flex flex-col gap-2">
  <h1 class="text-fc-2xl font-semibold">Welcome to Fusible</h1>
  <p class="text-fc-md text-fc-fg-muted">
    Boards, firmware and enclosures in one checked project. When a rule fails, the fuse blows and nothing ships.
  </p>
</div>
<ul class="flex flex-col gap-2">
  {#each features as feature (feature.icon)}
    <li class="flex items-center gap-3 rounded-fc-lg bg-fc-surface px-4 py-3">
      <Icon icon={feature.icon} size={24} />
      <span class="text-fc-sm">{feature.text}</span>
    </li>
  {/each}
</ul>
