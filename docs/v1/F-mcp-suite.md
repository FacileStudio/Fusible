# Track F: `fusible mcp`, Bulle, Opus and Nuage integration

Read `docs/v1/README.md` first.

Worktree: `/home/yann/Code/Kono/Fusible-v1-mcp`, branch `feat/v1-mcp`.

## Goal

Let any MCP client drive a Fusible project through the fuse loop: Bulle, Claude Code, Codex, Cursor. Every write returns the diff and the new fuse verdict.

Also connect two Facile apps by shelling out to their CLIs:
- **Opus:** a blown fuse or an unbought part becomes a task.
- **Nuage:** fab exports go to a synced space, with an optional share link.

## Paths you own

- `src/commands/mcp/` (new)
- `src/commands/share/` (new)
- `src/commands/opus.rs` (new)
- `src/commands/mod.rs`, additive only
- `src/cli.rs` / `src/cli/args.rs`: new `Mcp` and `Share` variants; `--opus` flag on `Check` and on `BomArgs`
- `src/main.rs` / `src/dispatch.rs`: route the new variants (`rg -n "Commands::" src/dispatch*`)
- `docs/agents.md` (new)
- `tests/mcp.rs`, `tests/share.rs`, `tests/opus.rs` (new), `tests/fixtures/bin/` (new fake CLIs)

Don't touch `src/linter/`, `src/dev/`, `studio/`, `desktop/` or `mobile/`.

No new crate dependency. JSON-RPC uses `serde_json`, which is already a dependency.

## Part 1: `fusible mcp`

### 1.1 Transport

- MCP over **stdio**: newline-delimited JSON-RPC 2.0 messages on stdin and stdout.
- stdout carries protocol messages only. Every log line goes to stderr.
- Read stdin line by line. Answer each request with the same `id`. Ignore notifications (no `id`).
- Run tool calls on the main thread, one at a time (fine for v1).
- `fusible mcp [--root <dir>]`: the project root defaults to the nearest ancestor of the cwd that holds `fusible.yml`. With none, the server still starts, and `project_info` returns `{"manifest": null}`.

### 1.2 Methods

| Method | Response |
|---|---|
| `initialize` | `{"protocolVersion": <client's version if it is one of "2025-11-25","2025-06-18","2025-03-26", else "2025-06-18">, "capabilities": {"tools": {"listChanged": false}, "resources": {"listChanged": false}}, "serverInfo": {"name": "fusible", "version": env!("CARGO_PKG_VERSION")}, "instructions": INSTRUCTIONS}` |
| `notifications/initialized` | none |
| `ping` | `{}` |
| `tools/list` | the tool list below, each `{name, description, inputSchema}` (JSON Schema draft 2020-12, `"type":"object"`, `required`, `additionalProperties:false`) |
| `tools/call` | `{"content":[{"type":"text","text":<pretty JSON>}], "structuredContent": <same JSON>, "isError": bool}` |
| `resources/list` | `fusible://project` (manifest as YAML text) and one `fusible://board/<name>/circuit.json` per board |
| `resources/read` | `{"contents":[{"uri","mimeType","text"}]}` |
| unknown method | error `-32601`. Bad params: `-32602`. Internal: `-32603`. |

`INSTRUCTIONS` (const string):

> "Fusible checks hardware designs. Call check after every edit. A blown fuse means the design breaks a rule: read each violation's message, standard and fix, change the board or part, and check again. export refuses to run while the fuse is blown. Edits are file changes in a git repository; nothing is committed for you."

### 1.3 Tools

These are exact names. Each tool lives in its own file under `src/commands/mcp/tools/`. Reuse the internal functions the CLI commands use. Don't spawn `fusible` as a subprocess.

| Tool | Input | Output (`structuredContent`) |
|---|---|---|
| `project_info` | `{}` | `{name, root, boards:[{name, entry, mcu}], firmware:[{name, board, framework}], bodies:[{name, kind, material, process}], git:{branch, dirty}}` |
| `check` | `{board?: string}` | `{fuse:"intact"\|"blown", boards:[{name, diagnostics:[...]}]}`, the same shape as `fusible check --json` (track D). Until D merges, call the existing lint path and map `Diagnostic` to `{rule, message, help, severity}`. |
| `explain_rule` | `{rule: string}` | `{id, summary, standard, source_url}` from `Linter::describe()` (track D). Until D merges, return `{id}` plus the rule's help text if known, else error `unknown rule`. |
| `list_components` | `{board: string}` | `[{name, kind, ports:[...]}]` from the parsed circuit |
| `list_nets` | `{board: string}` | `[{name, ports:[...], declared:{voltage?, current?}}]` |
| `bom` | `{}` | the BOM rows and totals, the same data `fusible bom --format json` produces (`rg -n "BomFormat" src`) |
| `firmware_pins` | `{firmware: string}` | the generated `pins.rs` text plus pin table, only when the fuse is intact. Otherwise `isError: true` with the violations. |
| `edit_board` | `{board: string, patch?: string, content?: string, dry_run?: bool (default true)}` | `{applied: bool, dry_run, diff, fuse, diagnostics}` |
| `body_new` | `{name: string, material?: string, process?: string}` | `{created:[paths]}`. Calls track E's `body new` function. Until E merges, return `isError` "body_new needs Fusible 1.0 bodies". |
| `edit_body` | `{name: string, content: string, dry_run?: bool}` | like `edit_board`, for a code-part `.ts` file |
| `build_body` | `{name: string}` | `{outputs:[paths], mass_g?, bbox?}` |
| `export` | `{board?: string, preset: "fab"\|"cad"\|"docs"\|"all"}` | `{files:[paths]}`. Refuses with `isError` and the violations when blown. |
| `render` | `{board: string, view: "pcb"\|"schematic"}` | SVG text from the native Rust SVG renderer (`src/svg/`, `rg -n "pub fn" src/svg`). If `magick` is on PATH, also an `{"type":"image","mimeType":"image/png","data":<base64>}` content item. Write the base64 encoder yourself in ~15 lines; no crate. |

### 1.4 Write safety (`edit_board`, `edit_body`)

Follow these steps in order. Test each one.

1. Resolve the target file from the manifest (`boards.<name>.entry` / `bodies.<name>.source`). Canonicalize it. Refuse it unless it is inside the project root (no `..`, no symlink escape).
2. Exactly one of `patch` or `content` must be given.
3. Refuse when `git status --porcelain -- <file>` shows changes not made by this server session. Keep a set of files this session wrote, and allow those.
4. Read the original bytes.
5. Apply the change.
   - For `patch`: write it to a temp file, run `git apply --check <tmp>`, then `git apply <tmp>`, from the project root.
   - For `content`: write the file.
6. Run the check for the affected board (bodies: run the mechanical rules for that body).
7. Get the diff with `git diff --no-color -- <file>`.
8. If `dry_run`: write the original bytes back. Do this in a guard, so it happens even when steps 5 to 7 fail.
9. Return `{applied: !dry_run && step 5 succeeded, dry_run, diff, fuse, diagnostics}`.

Nothing is ever committed.

### 1.5 `fusible mcp --print-config <bulle|claude|codex>`

Prints a snippet and exits 0.

- **bulle** (from `/home/yann/Code/Facile/Bulle/docs/configuration.md` around line 306, `sources.mcp` in `~/.config/bulle/config.yml`):

  ```yaml
  sources:
    mcp:
      fusible:
        command: fusible
        args: [mcp]
  ```

- **claude:** `claude mcp add fusible -- fusible mcp`
- **codex** (`~/.codex/config.toml`):

  ```toml
  [mcp_servers.fusible]
  command = "fusible"
  args = ["mcp"]
  ```

### 1.6 `docs/agents.md`

- What the MCP server does, with the tool table.
- The safety rules from 1.4.
- The three config snippets.
- A worked loop: `check`, then `edit_board` with dry run, then a real edit, then `check`, then `export`.
- Plain English, no emoji. Run the `unslop` skill on it.

## Part 2: Opus (`fusible check --opus`, `fusible bom --opus`)

- Detect `opus` on PATH. If it's missing, print to stderr `opus CLI not found; install it with: facile install opus`. **Keep the check's own exit code.** The Opus side is best effort and never changes the fuse verdict.
- **`check --opus`:**
  - For each blown-fuse **error** diagnostic, the key is `sha256` of `rule + "\n" + sorted elements + "\n" + board`. If track D's `elements` isn't merged yet, use the message.
  - Without a crypto crate: use `std::hash::DefaultHasher` hashed twice with different seeds, formatted as 32 hex chars. It's only for dedup.
  - Dedup file: `.fusible/opus.json`, `{key: task_id}`. Skip keys already present.
  - For a new key, run:

    ```sh
    opus task add "<board>: <message, max 120 chars>" --description "<rule>\n<help>\n<standard> <source_url>" --label fusible --label fuse --priority high --json
    ```

    and store the returned `id` (parse the JSON; on a parse failure, print a warning and store nothing).
  - Add `--project <name>` when `fusible.yml` has `opus: { project: <name> }`. Add that optional manifest key in `src/config/` only if you can do it in under 15 lines. Otherwise use a `--opus-project <name>` flag.
- **`bom --opus`:** for each BOM row with no price or `picked: false`, add `opus task add "Buy <qty> × <value> (<designators>)" --label fusible --label bom --json`. Use the same dedup file (key `bom:` + designators).
- **Never call `opus task update`, `done` or `delete`.**

## Part 3: Nuage (`fusible share nuage`)

`fusible share nuage --space <name> [--preset fab|cad|docs|all] [--link] [--expires 30d]`:

1. Run `nuage spaces list --json`. The output shape is `{"spaces":[{"id","name","role","sync_dir"}]}`. Find `name == --space`.
   - If `sync_dir` is null, exit 2 with "space <name> is not synced on this machine; run nuage to set a sync folder".
   - Expand a leading `~`.
2. Run the export for the preset (default `fab`) into a temp dir with the existing export function. A blown fuse means exit 1, and nothing is copied.
3. `rev` = `git rev-parse --short HEAD`, plus `-dirty` when the tree is dirty, or the crate version when the project isn't a git repo.
4. Copy the files to `<sync_dir>/Fusible/<project name>/<rev>/`. Refuse to overwrite an existing non-empty folder unless `--force` is given.
5. Run `nuage sync --json`. Non-zero means exit 2 with its stderr.
6. If `--link`: run `nuage share "Fusible/<project>/<rev>" --json --expires <expires>` and print the URL from the JSON.
   - The remote path format is unverified. If `nuage share` fails, print its stderr and say "files are synced; create the link from the Nuage app".
   - Exit 0 in that case: the files did arrive.
7. Print the local folder path and the number of files.

## Tests

The tests use **fake CLIs only**. Never call the real `opus` or `nuage`: they act on the user's real accounts.

- `tests/fixtures/bin/opus` and `tests/fixtures/bin/nuage`: small shell scripts (`#!/bin/sh`) that append their argv to `$FAKE_LOG` and print canned JSON. The tests put that folder first on `PATH`. Mark the tests `#[cfg(unix)]`.
- **`tests/mcp.rs`:** spawn `cargo_bin("fusible") mcp --root examples/<a project with fusible.yml>` (check which example has a manifest: `ls examples tests/fixtures`; create `tests/fixtures/mcp/` with a `fusible.yml` and a `circuit.json` copied from `examples/circuit.json` if none fits). Then:
  - `initialize`, then `tools/list` lists exactly the 13 tools;
  - `check` returns `fuse`;
  - `edit_board` with `dry_run` leaves the file bytes identical;
  - `edit_board` with a path outside the root is refused;
  - an unknown method gives `-32601`.
  - Use a temp copy of the fixture that you `git init`.
- **`tests/opus.rs`:** a blown fixture with `--opus` logs exactly one `task add`. Running it again logs none (dedup). Missing opus keeps exit 1 and prints the hint.
- **`tests/share.rs`:** the fake `nuage spaces list --json` returns a space whose `sync_dir` is a temp dir. The files land in `<tmp>/Fusible/<name>/<rev>/` and `nuage sync` is called. A space with `sync_dir: null` exits 2.

## Gates

```sh
cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings && cargo test --locked
filet check
printf '%s\n' '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"0"}}}' '{"jsonrpc":"2.0","id":2,"method":"tools/list"}' | cargo run -q -- mcp --root tests/fixtures/mcp | head -c 600
```

## Exit criteria

- All gates pass, and the MCP smoke run returns `initialize` and `tools/list`.
- The Bulle snippet matches Bulle's documented `sources.mcp` format.
- Opus and Nuage behave as specified against the fakes.
- Report written to `docs/v1/reports/F.md`. List any tool that is stubbed until D or E merge, so the integrator wires it after the merge.
