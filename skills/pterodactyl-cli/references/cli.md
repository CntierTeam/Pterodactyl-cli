# CLI command tree

```text
ptero config show|path|set-url|set-client-key|set-app-key|set-daemon-token
ptero account get|email|password|two-factor*|activity|api-keys|api-key-create|api-key-delete|ssh-keys|ssh-key-create|ssh-key-delete
ptero server list|permissions|get|resources|activity|power|command|websocket|startup|startup-var|startup-egg|rename|reinstall|docker-image
ptero file list|contents|download|write|rename|copy|compress|decompress|delete|mkdir|chmod|pull|upload-url
ptero db list|create|rotate|delete
ptero schedule list|get|create|update|execute|delete|task-create|task-update|task-delete
ptero network list|create|notes|primary|delete
ptero subuser list|get|create|update|delete
ptero backup list|get|create|download|lock|restore|delete
ptero terminal attach
ptero app user|node|location|server|nest|egg …
ptero remote sftp-auth|servers|servers-reset|activity|server|install|install-complete|transfer-*|backup-*
```

Global flags: `--url`, `--client-key`, `--app-key`, `--daemon-token`, `--json`, `--config`.

Env: `PTERO_URL`, `PTERO_CLIENT_KEY`, `PTERO_APP_KEY`, `PTERO_DAEMON_TOKEN`.

## Examples

```bash
ptero --url https://panel.example.com --client-key ptlc_... server list --json
ptero server power <id> start
ptero file list <id> /
ptero terminal attach <id>
ptero app user list --json
ptero remote servers
```

Power signals: `start`, `stop`, `restart`, `kill`.
