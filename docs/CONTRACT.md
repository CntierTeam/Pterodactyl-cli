# Pterodactyl-Cli Contract

Frozen public API between workspace crates. Cross-crate type changes go through `ptero-protocol` + this document.

## Binary behavior

- Binary: `ptero`
- **No argv → TUI** (`ptero-tui`)
- **Any subcommand/args → CLI** (`ptero-cli` / clap)

## Auth & transport

| Surface | Prefix | Authorization |
|---------|--------|---------------|
| Client | `/api/client` | `Bearer <client_api_key>` (typically `ptlc_…`) |
| Application | `/api/application` | `Bearer <application_api_key>` (typically `ptla_…`) |
| Remote | `/api/remote` | `Bearer <daemon_token_id>.<daemon_token_secret>` (Node daemon token) |

Common headers:

- `Accept: Application/vnd.pterodactyl.v1+json`
- `Content-Type: application/json` (except Client file write: `text/plain` body)

Fractal / JSON:API-style envelope (`ptero_protocol`):

```json
{ "object": "…", "attributes": { … } }
{ "object": "list", "data": [ … ], "meta": { "pagination": { … } } }
```

Websocket credentials (Client) are **not** Fractal attributes:

```json
{ "data": { "token": "…", "socket": "wss://…" } }
```

API error body:

```json
{ "errors": [ { "code": "…", "status": "…", "detail": "…" } ] }
```

Mapped to `PteroError::Api`.

Missing credentials must fail with a clear `PteroError::Config` message per surface (`client` / `application` / `remote`).

## Crate graph

```
bin/ptero → ptero-cli + ptero-tui
ptero-cli / ptero-tui → ptero-core
ptero-core → ptero-panel + ptero-wings + ptero-config
ptero-panel / ptero-wings / ptero-config → ptero-protocol
```

**Rule:** CLI/TUI must not call HTTP/WS directly; only via `ptero-core` services.

## `ptero-protocol`

- Fractal envelopes: `FractalItem`, `FractalList`, `Pagination`, `Meta`
- `ApiError` / `ApiErrorItem`, `PteroError`, `PteroResult`
- WS event / request constants (`ws` module) + `WsMessage`
- Domain DTOs: `account`, `server`, `files`, `schedule`, `network`, `backup`, `application`, `remote`
- `WebsocketCredentials` / `WebsocketResponse`

## `ptero-config`

- Path: `~/.config/ptero/config.toml`
- Fields: `panel_url`, `client_api_key`, `application_api_key`, `daemon_token`
- Helpers: load/save, env/CLI override application, `is_ready_*` per API surface

## `ptero-panel`

Traits (implemented by `HttpPanelClient`):

- `ClientApi` — full Client surface + `raw`
- `ApplicationApi` — full Application surface + `raw`
- `RemoteApi` — full Remote surface + `raw`

Escape hatch: `raw(method, path, query, body)` selects Bearer credentials by trait / surface.

## `ptero-wings`

Wings console WebSocket:

1. Client `GET /api/client/servers/{id}/websocket` → `{ data: { token, socket } }`
2. Connect WS; message shape `{ event, args }`
3. Send `auth` with token; refresh on `token expiring` / `token expired`
4. Client→server: `send command`, `set state`, `send logs`

## `ptero-core`

- `AppContext` / `AppContextBuilder`
- Services: `AccountService`, `ServerService`, `FileService`, `DatabaseService`, `ScheduleService`, `NetworkService`, `SubuserService`, `BackupService`, `ApplicationService`, `RemoteService`, `TerminalService`

## CLI tree

```
ptero config|account|server|file|db|schedule|network|subuser|backup|terminal|app|remote
```

Global: `--url`, `--client-key`, `--app-key`, `--daemon-token`, `--json`, `--config`.

## Source of truth

Reference clone (not vendored): `/tmp/pterodactyl-china-panel` branch `1.0-develop`

- `routes/api-client.php`
- `routes/api-application.php`
- `routes/api-remote.php`
- Frontend WS: `resources/scripts/plugins/Websocket.ts`
- Remote auth middleware: `DaemonAuthenticate`

## Out of scope (by design)

- Panel Web UI / install scripts
- Vendoring the panel clone into this repo
- Full Wings daemon replacement (Remote CLI is for debug / simulation of Panel internal APIs)
