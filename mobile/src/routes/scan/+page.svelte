<script lang="ts">
  import { goto } from "$app/navigation";
  import { Alert, Button, Input, Spinner } from "@facile/muse";
  import {
    CapacitorBarcodeScanner,
    CapacitorBarcodeScannerAndroidScanningLibrary as Lib,
    CapacitorBarcodeScannerTypeHint as Hint,
  } from "@capacitor/barcode-scanner";
  import { addHost } from "$lib/hosts";
  import { claim, parsePairUri } from "$lib/pair";
  import { ICON } from "$lib/icons";
  import { useT, type Key } from "$lib/i18n";

  const t = useT();
  let busy = $state(false);
  let error = $state<Key | null>(null);
  let pasted = $state("");

  const failures: Record<string, Key> = {
    not_pair_code: "errNotCode",
    not_private: "errNotPrivate",
    bad_token: "errBadToken",
    claim_401: "errClaim401",
    claim_429: "errClaim429",
  };

  async function pair(uri: string) {
    const parsed = parsePairUri(uri);
    if (!parsed.ok) return void (error = failures[parsed.error] ?? "errNotCode");
    busy = true;
    try {
      const device = await claim(parsed.host, parsed.token, "Fusible phone");
      const id = device.device_id;
      const paired_at = new Date().toISOString();
      await addHost({ id, name: parsed.name, url: parsed.host, token: device.device_token, paired_at });
      await goto(`/host?id=${encodeURIComponent(id)}`);
    } catch (e) {
      error = failures[(e as Error).message] ?? "errClaim";
    } finally {
      busy = false;
    }
  }

  async function scan() {
    error = null;
    try {
      const result = await CapacitorBarcodeScanner.scanBarcode({
        hint: Hint.QR_CODE,
        android: { scanningLibrary: Lib.ZXING },
      });
      await pair(result.ScanResult);
    } catch {
      error = "errNotCode";
    }
  }
</script>

<main class="mx-auto flex max-w-xl flex-col gap-4 px-4 pt-8 pb-10">
  <Button variant="ghost" href="/" icon={ICON.back} class="self-start">{t("back")}</Button>
  <h1 class="text-fc-2xl font-semibold">{t("scanTitle")}</h1>
  {#if error}
    <Alert tone="danger">{t(error)}</Alert>
  {/if}
  <Button size="lg" icon={ICON.scanner} disabled={busy} onclick={scan}>{t("scanAction")}</Button>
  <label class="flex flex-col gap-2 text-fc-sm text-fc-fg-muted">
    {t("pasteLabel")}
    <Input bind:value={pasted} placeholder="fusible://pair?h=..." class="border-0 bg-fc-surface" />
  </label>
  <Button size="lg" variant="ghost" disabled={busy || !pasted} onclick={() => pair(pasted)}>
    {t("pasteAction")}
  </Button>
  {#if busy}
    <div class="flex items-center gap-2 text-fc-fg-muted"><Spinner /> {t("pairing")}</div>
  {/if}
</main>
