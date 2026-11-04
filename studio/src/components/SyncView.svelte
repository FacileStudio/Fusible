<script lang="ts">
  import { Button } from "@facile/muse";
  import { actGit, ago, git, recheckGit, summary } from "../lib/git.svelte";

  const s = $derived(git.status);
  const stuck = $derived((s?.conflicts.length ?? 0) > 0);
  const cannotSave = $derived(git.busy || stuck || !s?.changes.length);
  const cannotUpdate = $derived(git.busy || stuck || !s?.upstream);
</script>

{#if s?.repo}
  <section class="flex flex-col gap-2 rounded-3xl bg-fc-component p-4 text-fc-sm" aria-label="Sync">
    <div class="flex items-center justify-between">
      <span class="font-medium">Sync</span>
      <Button variant="ghost" size="sm" icon="solar:refresh-circle-linear" disabled={git.busy} onclick={recheckGit}>Check</Button>
    </div>
    {#if s.available === false}
      <p>Install Git to sync your work.</p>
      <p class="text-fc-fg-muted">Git is the free program that shares your project with your team. Install it, then come back.</p>
      <a class="text-fc-info underline" href="https://git-scm.com/downloads" target="_blank" rel="noopener noreferrer">Download Git</a>
    {:else}
      <p id="sync-status">{summary(s)}</p>
      {#if !s.has_remote}<p class="text-fc-fg-muted">Saved on this computer only. Add a remote to share.</p>{/if}
      {#if s.fetch_error}<p class="text-fc-fg-muted">Could not check for updates: {s.fetch_error}</p>{/if}
      {#each s.conflicts as path (path)}
        {@const author = s.conflict_authors[path]}
        <div class="flex flex-col gap-2 rounded-2xl bg-fc-danger/10 p-3">
          <span>{author ? `${author} and you both changed ` : "Both you and someone else changed "}<b class="break-all">{path}</b>. Pick the version to keep.</span>
          <div class="flex gap-1">
            <Button size="sm" variant="ghost" icon="solar:check-circle-linear" onclick={() => actGit("resolve", { path, keep: "mine" })}>Keep mine</Button>
            <Button size="sm" variant="ghost" icon="solar:download-minimalistic-linear" onclick={() => actGit("resolve", { path, keep: "theirs" })}>Keep theirs</Button>
          </div>
        </div>
      {/each}
      {#if s.changes.length}
        <textarea
          class="min-h-14 w-full resize-y rounded-2xl bg-fc-surface p-3 text-fc-sm select-text focus-visible:outline-2 focus-visible:outline-fc-ring"
          placeholder="What did you change?"
          aria-label="What did you change?"
          bind:value={git.draft}
        ></textarea>
      {/if}
      <div class="flex gap-1">
        <Button size="sm" icon="solar:diskette-linear" disabled={cannotSave} onclick={() => actGit("save", { message: git.draft })}>
          {git.busy ? "Working..." : "Save"}
        </Button>
        <Button size="sm" variant="ghost" icon="solar:download-minimalistic-linear" disabled={cannotUpdate} onclick={() => actGit("update")}>
          Get updates
        </Button>
      </div>
      {#if git.notice}
        <div class="rounded-2xl p-3 {git.notice.error ? 'bg-fc-danger/10 text-fc-danger' : 'bg-fc-success/10'}" role={git.notice.error ? "alert" : "status"}>
          {git.notice.text}
          {#if git.notice.details}<details><summary>Details</summary><pre class="mt-2 whitespace-pre-wrap text-fc-fg-muted select-text">{git.notice.details}</pre></details>{/if}
        </div>
      {/if}
      {#if s.changes.length}
        <details>
          <summary class="cursor-pointer text-fc-fg-muted">Changes ({s.changes.length})</summary>
          {#each s.changes as change (change.path)}
            <div class="flex gap-3 py-1 text-fc-xs"><span class="w-14 shrink-0 text-fc-fg-muted">{change.kind}</span><span class="break-all">{change.path}</span></div>
          {/each}
        </details>
      {/if}
      {#if s.last_saves.length}
        <details>
          <summary class="cursor-pointer text-fc-fg-muted">Saves ({s.last_saves.length})</summary>
          {#each s.last_saves as save (save.when + save.message)}
            <div class="py-1 text-fc-xs" title={save.files.join("\n")}>
              {save.message}<span class="block text-fc-fg-muted">{save.author} &middot; {ago(save.when)}</span>
            </div>
          {/each}
        </details>
      {/if}
    {/if}
  </section>
{/if}
