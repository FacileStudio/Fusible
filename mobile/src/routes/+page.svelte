<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { Button, EmptyState } from "@facile/muse";
  import HostCard from "../components/HostCard.svelte";
  import Onboarding from "../components/Onboarding.svelte";
  import { type Host, isOnboarded, loadHosts, removeHost, setOnboarded } from "$lib/hosts";
  import { mockHost } from "$lib/fixtures";
  import { ICON } from "$lib/icons";
  import { useT } from "$lib/i18n";

  const t = useT();
  let hosts = $state<Host[]>([]);
  let onboarded = $state<boolean | null>(null);

  onMount(async () => {
    const mock = new URLSearchParams(location.search).get("mock") === "1";
    hosts = [...(await loadHosts()), ...(mock ? [mockHost] : [])];
    onboarded = mock || (await isOnboarded());
  });

  async function finish(scan: boolean) {
    await setOnboarded();
    onboarded = true;
    if (scan) await goto("/scan");
  }

  async function forget(id: string) {
    await removeHost(id);
    hosts = hosts.filter((h) => h.id !== id);
  }
</script>

{#if onboarded === false}
  <Onboarding ondone={finish} />
{:else if onboarded}
  <main class="mx-auto flex max-w-xl flex-col gap-4 px-4 pt-8 pb-10">
    <h1 class="text-fc-2xl font-semibold">{t("appName")}</h1>
    {#each hosts as host (host.id)}
      <HostCard {host} onforget={forget} />
    {:else}
      <EmptyState icon={ICON.phone} title={t("noHosts")} description={t("noHostsBody")} bare />
    {/each}
    <Button size="lg" href="/scan" icon={ICON.qr}>{t("pairStudio")}</Button>
  </main>
{/if}
