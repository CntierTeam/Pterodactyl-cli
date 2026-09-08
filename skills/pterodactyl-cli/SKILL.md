---
name: pterodactyl-cli
description: >-
  Develop and operate the Pterodactyl-Cli Rust workspace (ptero binary: TUI with no args,
  CLI with subcommands). Covers Client / Application / Remote Panel APIs, Wings console WS,
  crate contracts (ptero-protocol/panel/wings/core/cli/tui), and FEATURE_PARITY.
  Trigger on: Pterodactyl-Cli, ptero, Pterodactyl Panel API, Wings websocket, Client/Application/Remote API.
license: MIT
metadata:
  short-description: Pterodactyl Rust CLI/TUI workspace guide
---

# Pterodactyl-Cli

Rust workspace client for Pterodactyl Panel (Client + Application + Remote) and Wings console. Binary: `ptero`.

## Hard rules

1. **No args → TUI**; **any subcommand → CLI**.
2. CLI/TUI **must not** call HTTP/WS directly — only through `ptero-core` services.
3. Cross-crate types live only in `ptero-protocol`. Changing a public trait/DTO requires updating `docs/CONTRACT.md`.
4. Three credential surfaces: Client (`ptlc_`), Application (`ptla_`), Remote (`id.secret` daemon token).
5. Code and comments in English; user-facing agent replies follow the user's language.
6. Do **not** vendor `/tmp/pterodactyl-china-panel` into the repo.

## Workspace map

| Crate | Role |
|-------|------|
| `ptero-protocol` | Fractal envelopes, errors, DTOs, WS constants |
| `ptero-panel` | `ClientApi` / `ApplicationApi` / `RemoteApi` + `HttpPanelClient` |
| `ptero-wings` | Wings console WebSocket (auth / command / token refresh) |
| `ptero-config` | `~/.config/ptero/config.toml` |
| `ptero-core` | Domain services shared by CLI/TUI |
| `ptero-cli` | clap commands |
| `ptero-tui` | ratatui UI |
| `bin/ptero` | Entry |

Panel envelope: Fractal `{ object, attributes | data[], meta.pagination }`.  
Websocket credentials: `{ data: { token, socket } }` (not Fractal attributes).

## Common workflows

### Build / run

```bash
cargo build -p ptero
./target/debug/ptero --help
./target/debug/ptero          # TUI
```

### Configure

```bash
ptero config set-url https://panel.example.com
ptero config set-client-key ptlc_...
ptero config set-app-key ptla_...
ptero config set-daemon-token id.secret
```

Or one-shot: `ptero --url ... --client-key ... --json server list`.

### Install Codex skill

```bash
./scripts/install-codex-skill.sh        # copy → ~/.codex/skills/pterodactyl-cli
./scripts/install-codex-skill.sh link   # symlink
```

### Extending API coverage

1. Add DTO in `ptero-protocol` if needed.
2. Add method on the matching trait + `HttpPanelClient` (mirror panel `routes/api-*.php`).
3. Wire `ptero-core` service.
4. Expose clap subcommand in `ptero-cli` and/or TUI screen.
5. Update `docs/CONTRACT.md` and `docs/FEATURE_PARITY.md`.

Truth sources: `routes/api-client.php`, `api-application.php`, `api-remote.php`; WS: `resources/scripts/plugins/Websocket.ts`.

## Parallel / subagent work

Change only your crate. Do not edit another crate's internals. Protocol-breaking changes must be explicit and documented in `docs/CONTRACT.md`.

## References

- Full crate API map: [references/contract.md](references/contract.md)
- CLI command tree: [references/cli.md](references/cli.md)
- Project docs: `docs/CONTRACT.md`, `docs/FEATURE_PARITY.md`, `README.md`
- Architecture sibling: MCSManager-CLI (same CLI/TUI → core pattern; different protocol)
