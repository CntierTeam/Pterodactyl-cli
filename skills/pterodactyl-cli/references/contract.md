# Contract summary (load when changing crate boundaries)

## Binary

- `ptero` with no argv → TUI
- `ptero <subcommand>` → CLI

## Auth

| Surface | Prefix | Authorization |
|---------|--------|---------------|
| Client | `/api/client` | `Bearer <client_api_key>` (`ptlc_…`) |
| Application | `/api/application` | `Bearer <application_api_key>` (`ptla_…`) |
| Remote | `/api/remote` | `Bearer <id>.<secret>` (Node daemon token) |

Headers: `Accept: Application/vnd.pterodactyl.v1+json`.

## Graph

```
bin/ptero → ptero-cli + ptero-tui
ptero-cli / ptero-tui → ptero-core
ptero-core → ptero-panel + ptero-wings + ptero-config
* → ptero-protocol
```

**Rule:** CLI/TUI must not call HTTP/WS directly.

## Envelope (Fractal)

```json
{ "object": "…", "attributes": { } }
{ "object": "list", "data": [ ], "meta": { "pagination": { } } }
```

Websocket credentials (Client) are not Fractal attributes:

```json
{ "data": { "token": "…", "socket": "wss://…" } }
```

API errors → `PteroError::Api` from `{ "errors": [ … ] }`.

## Wings console

Client websocket credentials → Wings console WS (`ptero-wings`). Auth / command / token refresh live there; TUI terminal screen uses core services only.
