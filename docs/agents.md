# Drive Fusible from an AI agent

`fusible mcp` is a Model Context Protocol server. It lets an agent such as Bulle, Claude Code, Codex or Cursor read a hardware project, edit it and check it in the same fuse loop a human uses.

The server speaks JSON-RPC 2.0 over stdio, one message per line. Stdout carries protocol messages only. Logs go to stderr.

## Start the server

```sh
fusible mcp                 # project root: nearest folder with a fusible.yml
fusible mcp --root ./board  # explicit root
```

Without a `fusible.yml` the server still starts, and `project_info` returns `{"manifest": null}`.

## Tools

| Tool | Input | What it does |
|---|---|---|
| `project_info` | none | Boards, firmware, bodies and git state. |
| `check` | `board?` | Runs the rules. Returns `fuse` (`intact` or `blown`) and the violations. |
| `explain_rule` | `rule` | Severity of a rule id. Summary, standard and source URL arrive with the rules track. |
| `list_components` | `board` | Components with kind and ports. |
| `list_nets` | `board` | Nets with the ports they join. |
| `bom` | none | The bill of materials, same data as `fusible bom --format json`. |
| `firmware_pins` | `firmware` | The generated `pins.rs` and a pin table. Refuses while the fuse is blown. |
| `edit_board` | `board`, `patch` or `content`, `dry_run?` | Edits the board source, checks it and returns the diff. |
| `body_new` | `name`, `material?`, `process?` | Creates a code-first body. Needs Fusible 1.0 bodies. |
| `edit_body` | `name`, `content`, `dry_run?` | Edits the `.ts` source of a code-first body. |
| `build_body` | `name` | Builds one body and lists its files. |
| `export` | `board?`, `preset` | Writes `fab`, `cad`, `docs` or `all` files. Refuses while the fuse is blown. |
| `render` | `board`, `view` | SVG of the PCB or schematic. Adds a PNG when ImageMagick is installed. |

Every tool returns text content and `structuredContent`. List tools wrap their array in an object, for example `{"components": [...]}`, because MCP requires structured content to be an object.

The server also exposes two kinds of resource: `fusible://project` (the manifest) and `fusible://board/<name>/circuit.json`.

## Write safety

`edit_board` and `edit_body` change files in your project. They follow these rules, in this order.

1. The target comes from the manifest and must resolve, after symlinks, inside the project root.
2. Give exactly one of `patch` (unified diff) or `content` (whole file).
3. The file must have no uncommitted changes, unless this server session wrote them and nobody touched the file since.
4. The server reads the original bytes.
5. It applies the change. A patch runs through `git apply --check`, then `git apply`, and may only name the target file.
6. It runs the check for the board.
7. It captures `git diff` for the file.
8. With `dry_run` (the default), it writes the original bytes back, even when a step failed.
9. It returns `applied`, `dry_run`, `diff`, `fuse` and `diagnostics`.

The server never commits. The project must be a git repository.

## Configure a client

Bulle, in `~/.config/bulle/config.yml`:

```yaml
sources:
  mcp:
    fusible:
      command: fusible
      args: [mcp]
```

Claude Code:

```sh
claude mcp add fusible -- fusible mcp
```

Codex, in `~/.codex/config.toml`:

```toml
[mcp_servers.fusible]
command = "fusible"
args = ["mcp"]
```

`fusible mcp --print-config bulle|claude|codex` prints the same snippets.

## A worked loop

1. Call `check`. The reply says `"fuse": "blown"` and lists a floating net.
2. Call `edit_board` with the fix as `content` and the default `dry_run: true`. Read `diff` and `fuse`. The file on disk is unchanged.
3. When the preview shows `"fuse": "intact"`, call `edit_board` again with `dry_run: false`.
4. Call `check` again to confirm.
5. Call `export` with `preset: "fab"`. It returns the file paths. While the fuse is blown it returns an error and the violations instead.

## Opus and Nuage

Two CLI flags connect other Facile apps. Both are best effort. They never change the fuse verdict or the exit code.

- `fusible check --opus [--opus-project <name>]` opens one Opus task per new blown-fuse error. `fusible bom --opus` opens a "Buy" task for each part with no price or not picked. Fusible records known tasks in `.fusible/opus.json`, so a rerun creates nothing new.
- `fusible share nuage --space <name> [--preset fab|cad|docs|all] [--link] [--expires 30d] [--force]` exports behind the fuse, copies the files to `<sync folder>/Fusible/<project>/<revision>/` and runs `nuage sync`. With `--link` it also asks Nuage for a share link.
