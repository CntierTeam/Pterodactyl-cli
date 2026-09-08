# Feature parity checklist

Mapped from pterodactyl-china Panel routes (`/tmp/pterodactyl-china-panel/routes/api-*.php`) → `ptero` CLI/TUI.

Status legend: `todo` | `stub` | `done` | `out of scope`

## Transport / config (S0–S1)

| Area | CLI | TUI | Status |
|------|-----|-----|--------|
| Workspace / binary routing | `ptero` | no-args | done |
| Fractal envelope + ApiError | — | — | done |
| Config three credentials | `config show\|set-*\|path` | Help tip | done |
| HttpPanelClient auth dispatch | `raw` + typed methods | — | done |

## Client API (`routes/api-client.php`) — S2

| Area | CLI | TUI | Status |
|------|-----|-----|--------|
| List servers / permissions | `server list\|permissions` | Servers | done |
| Account / 2FA / email / password | `account *` | Account | done |
| API keys / SSH keys / activity | `account *` | — | done |
| Server get / resources / activity | `server get\|resources\|activity` | Servers | done |
| Power / command | `server power\|command` | o/s/r | done |
| Websocket credentials | `server websocket` / terminal | Terminal | done |
| Databases | `db *` | — | done |
| Files | `file *` | — | done |
| Schedules / tasks | `schedule *` | — | done |
| Network allocations | `network *` | — | done |
| Subusers | `subuser *` | — | done |
| Backups | `backup *` | — | done |
| Startup / settings | `server startup\|rename\|reinstall\|docker-image` | — | done |

Note (China fork): 2FA disable is `POST /api/client/account/two-factor/disable`.

## Application API (`routes/api-application.php`) — S3

| Area | CLI | TUI | Status |
|------|-----|-----|--------|
| Users CRUD / external | `app user *` | App Users | done |
| Nodes CRUD / deployable / configuration | `app node *` | App Nodes | done |
| Node allocations | `app node allocation*` | — | done |
| Locations CRUD | `app location *` | — | done |
| Servers CRUD / details / build / startup | `app server *` | — | done |
| Suspend / unsuspend / reinstall | `app server *` | — | done |
| Server databases | `app server db*` | — | done |
| Nests / eggs | `app nest\|egg *` | — | done |

No Mounts/Roles routes in current `api-application.php` snapshot (1.0-develop).

## Remote API (`routes/api-remote.php`) — S3b

| Area | CLI | TUI | Status |
|------|-----|-----|--------|
| SFTP auth | `remote sftp-auth` | — | done |
| Servers list / reset | `remote servers\|servers-reset` | — | done |
| Activity ingest | `remote activity` | — | done |
| Server details / install / transfer | `remote server\|install\|transfer-*` | — | done |
| Backup upload / status / restore | `remote backup-*` | — | done |

Auth: Node `daemon_token_id.daemon_token` via `DaemonAuthenticate` (not ptla/ptlc).

## Wings console WS — S4

| Area | CLI | TUI | Status |
|------|-----|-----|--------|
| Attach / send command / set state | `terminal attach` | Enter on server | done |
| Token refresh | automatic | automatic (on attach) | done |

## TUI screens — S5

| Screen | Key | Status |
|--------|-----|--------|
| Home | `0` | done |
| Servers | `1` | done |
| Account | `2` | done |
| App Users | `3` | done |
| App Nodes | `4` | done |
| Config / Help | `5` | done |
| Terminal | Enter from Servers | done (snapshot + HTTP command send) |

## Out of scope

| Area | Status |
|------|--------|
| Panel Web UI / install | out of scope |
| Vendoring panel into git | out of scope |
| Full Wings daemon replacement | out of scope |
