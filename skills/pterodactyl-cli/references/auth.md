# Auth surfaces

`ptero` talks to three Panel API prefixes. Use the matching credential or requests fail.

| Surface | Path prefix | Credential | Header |
|---------|-------------|------------|--------|
| **Client** | `/api/client` | Client API key `ptlc_…` | `Authorization: Bearer <key>` |
| **Application** | `/api/application` | Application API key `ptla_…` | `Authorization: Bearer <key>` |
| **Remote** | `/api/remote` | Node daemon token `id.secret` | `Authorization: Bearer <id>.<secret>` |

Accept header (handled by the client): `Application/vnd.pterodactyl.v1+json`.

## Which commands need which key

| Need | Credential | Top-level commands |
|------|------------|--------------------|
| Own account, servers you can access, files, console | Client (`ptlc_`) | `account`, `server`, `file`, `db`, `schedule`, `network`, `subuser`, `backup`, `terminal` |
| Panel admin: users, nodes, locations, create servers, nests/eggs | Application (`ptla_`) | `app …` |
| Wings/node daemon-facing remote endpoints | Daemon token (`id.secret`) | `remote …` |

You can store all three in config; only the relevant one is sent per command family.

## Where keys come from

- **Client / Application keys:** Panel → Account (or Admin) → API Credentials. Create with appropriate permissions.
- **Daemon token:** Node configuration / Wings config (`token_id` + `token` → `id.secret`). Not a `ptl*` user key.

## Config readiness

```bash
ptero config show
```

TUI Help (`5`) shows whether client / app / remote credentials are set (not the secret values).

## Operational tips

- Prefer saving keys with `ptero config set-client-key` / `set-app-key` / `set-daemon-token` once.
- For scripts, env vars (`PTERO_CLIENT_KEY`, etc.) beat putting secrets on the argv that ends up in `ps`/history — still treat env as sensitive.
- Websocket console: Client API returns `{ data: { token, socket } }`; `ptero terminal attach` uses that path for you.
- China / 翼龙 panels: same Client/Application/Remote model; set `--url` to that panel’s HTTPS origin.
