---
name: pterodactyl-cli
description: >-
  Use and operate Pterodactyl / 翼龙 Panel via the `ptero` binary (TUI with no args,
  CLI with subcommands). Covers install, config keys (Client / Application / Remote),
  common panel workflows (servers, power, files, terminal, app admin), and TUI keys.
  Trigger on: ptero, Pterodactyl-Cli, Pterodactyl Panel, 翼龙, Wings console,
  client/app/remote API keys, panel server list/power/files/terminal.
license: MIT
metadata:
  short-description: Operate Pterodactyl Panel with ptero CLI/TUI
---

# Pterodactyl-Cli (`ptero`) — usage & operations

Operate a [Pterodactyl Panel](https://github.com/pterodactyl/panel) (including
[pterodactyl-china/panel](https://github.com/pterodactyl-china/panel) / 翼龙) with the
`ptero` CLI/TUI. Repo: https://github.com/CntierTeam/Pterodactyl-cli

**This skill is for using the tool against a panel — not for extending the Rust crates.**
Contributors: see the repo `README.md` and `docs/`.

## When to use

Trigger when the user wants to:

- Install or upgrade `ptero`
- Point `ptero` at a panel URL and API keys
- List / power / rename servers, manage files, DBs, schedules, backups
- Attach to Wings console (`terminal attach` or TUI)
- Use Application API admin flows (`ptero app …`) or Remote/daemon flows (`ptero remote …`)

## Hard rules (agent)

1. **No args → TUI**; **any subcommand → CLI**. Prefer CLI + `--json` for automation.
2. **Do not invent flags or endpoints.** Truth: `ptero <cmd> --help` and [references/cli.md](references/cli.md).
3. Prefer **`--json`** for machine-readable output; parse that instead of guessing fields.
4. Pick the right **auth surface** (Client / Application / Remote) — see [references/auth.md](references/auth.md). Wrong key → 401/403.
5. **Secrets:** never echo full API keys or daemon tokens into chat logs, commit messages, or pasted command history. Prefer `ptero config set-*` once, then omit keys from later commands. If a one-shot flag is required, redact in summaries.
6. Destructive actions (`power kill`, `reinstall`, `delete`, `backup restore`, app server delete) — confirm intent with the user when ambiguous.
7. User-facing replies follow the user's language.

## Install

Binary name: `ptero`. Prebuilts: [Releases](https://github.com/CntierTeam/Pterodactyl-cli/releases)
(`main` pushes update **Continuous** pre-release; `v*` tags = stable).

### Linux / macOS

Stable (latest non-prerelease) → `~/.local/bin`:

```bash
curl -fsSL https://raw.githubusercontent.com/CntierTeam/Pterodactyl-cli/main/scripts/install.sh | bash
```

Track Continuous from `main`:

```bash
curl -fsSL https://raw.githubusercontent.com/CntierTeam/Pterodactyl-cli/main/scripts/install.sh | bash -s -- --continuous
```

Pin version / custom prefix:

```bash
curl -fsSL https://raw.githubusercontent.com/CntierTeam/Pterodactyl-cli/main/scripts/install.sh | bash -s -- --version v0.1.0
PREFIX=/usr/local curl -fsSL https://raw.githubusercontent.com/CntierTeam/Pterodactyl-cli/main/scripts/install.sh | bash
```

From a local clone: `./scripts/install.sh [--continuous|--version vX.Y.Z]`.

Ensure `~/.local/bin` (or `$PREFIX/bin`) is on `PATH`, then:

```bash
command -v ptero && ptero --help
```

### Windows (PowerShell)

Default install: `%LOCALAPPDATA%\Programs\ptero` (+ user PATH):

```powershell
irm https://raw.githubusercontent.com/CntierTeam/Pterodactyl-cli/main/scripts/install.ps1 | iex
```

Continuous / pin:

```powershell
& ([scriptblock]::Create((irm https://raw.githubusercontent.com/CntierTeam/Pterodactyl-cli/main/scripts/install.ps1))) -Continuous
& ([scriptblock]::Create((irm https://raw.githubusercontent.com/CntierTeam/Pterodactyl-cli/main/scripts/install.ps1))) -Version v0.1.0
```

Local: `.\scripts\install.ps1 [-Continuous|-Version vX.Y.Z]`. Asset: **windows-amd64**.

### From source (optional)

```bash
cargo build --release -p ptero
./target/release/ptero --help
# or: cargo install --path bin/ptero
```

## Config

Default file: `~/.config/ptero/config.toml`

```bash
ptero config set-url https://panel.example.com
ptero config set-client-key ptlc_...      # Client API — most user workflows
ptero config set-app-key ptla_...         # Application API — admin
ptero config set-daemon-token id.secret   # Remote / Wings node token
ptero config show
ptero config path
```

Overrides (flag → env → config file):

| Flag | Env | Used for |
|------|-----|----------|
| `--url` | `PTERO_URL` | Panel base URL |
| `--client-key` | `PTERO_CLIENT_KEY` | `/api/client` |
| `--app-key` | `PTERO_APP_KEY` | `/api/application` |
| `--daemon-token` | `PTERO_DAEMON_TOKEN` | `/api/remote` |
| `--json` | — | JSON output |
| `--config` | — | Alternate config path |

One-shot (avoid leaving keys in shell history when possible):

```bash
ptero --url https://panel.example.com --client-key ptlc_... --json server list
```

## Auth surfaces (quick)

| Surface | Key shape | Commands |
|---------|-----------|----------|
| **Client** | `ptlc_…` | `account`, `server`, `file`, `db`, `schedule`, `network`, `subuser`, `backup`, `terminal` |
| **Application** | `ptla_…` | `app user\|node\|location\|server\|nest\|egg` |
| **Remote** | `id.secret` (node daemon) | `remote …` |

Details: [references/auth.md](references/auth.md).

## Common workflows

### List servers & power

```bash
ptero server list --json
ptero server get <id|uuid>
ptero server resources <id>
ptero server power <id> start    # start | stop | restart | kill
ptero server command <id> "say hello"
```

### Files

```bash
ptero file list <id> /
ptero file contents <id> /server.properties
ptero file write <id> /motd.txt --contents "hello"
ptero file write <id> /foo.cfg --from-file ./local.cfg
ptero file mkdir <id> / plugins
ptero file compress <id> / --files "world,world_nether"
ptero file pull <id> https://example.com/plugin.jar --directory /plugins
```

### Wings console

```bash
ptero terminal attach <id>
# or TUI: ptero → 1 (Servers) → Enter
ptero server websocket <id>   # credentials only (token + socket URL)
```

### Account / keys

```bash
ptero account get --json
ptero account api-keys
ptero account activity
```

### Application admin (needs `ptla_`)

```bash
ptero app user list --json
ptero app node list --json
ptero app nest list --json
ptero app egg list <nest_id> --json
ptero app server list --json
ptero app server create --json-body '{"name":"…", …}'   # see Panel Application API docs for body
ptero app server suspend <id>
ptero app server unsuspend <id>
```

### Remote / daemon (needs `id.secret`)

```bash
ptero remote servers
ptero remote server <uuid>
ptero remote install <uuid>
```

Full tree + more examples: [references/cli.md](references/cli.md).

## TUI

```bash
ptero    # no subcommand
```

| Key | Action |
|-----|--------|
| `0` | Home |
| `1` | Servers (`j`/`k` or arrows; `o` start, `s` stop, `r` restart; Enter → terminal) |
| `2` | Account |
| `3` | App Users (needs app key) |
| `4` | App Nodes (needs app key) |
| `5` / `h` / `?` | Help / Config status |
| `R` | Refresh (non-terminal) |
| `Esc` | Leave terminal screen |
| `q` | Quit (not while typing in terminal) |

In terminal screen: type + Enter sends a console command; Esc returns to Servers.

## Agent recipes

1. Resolve binary: `command -v ptero` (or `./target/release/ptero` after local build).
2. Ensure URL + needed key: `ptero config show` (redact keys when reporting).
3. Run the smallest command that answers the user; add `--json` for parsing.
4. On auth errors, check surface mismatch (client vs app vs remote) before retrying.
5. Do not invent REST paths — wrap `ptero` only.

## References

- Auth surfaces & when to use which key: [references/auth.md](references/auth.md)
- CLI command tree & examples: [references/cli.md](references/cli.md)
- Upstream Panel API: Pterodactyl Panel docs / your panel’s API keys page
