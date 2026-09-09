# CLI command tree

Skill 为 execute-first 操作员代跑；本文件是准确命令树备查，不是替代 shell 执行。不要发明子命令；不确定跑 `ptero <cmd> --help`。

```text
ptero                          # TUI（无子命令）
ptero --help

ptero config show|path|set-url|set-client-key|set-app-key|set-daemon-token

ptero account get|email|password|two-factor|two-factor-enable|two-factor-disable
              |activity|api-keys|api-key-create|api-key-delete
              |ssh-keys|ssh-key-create|ssh-key-delete

ptero server list|permissions|get|resources|activity|power|command|websocket
               |startup|startup-var|startup-egg|rename|reinstall|docker-image

ptero file list|contents|download|write|rename|copy|compress|decompress
             |delete|mkdir|chmod|pull|upload-url

ptero db list|create|rotate|delete
ptero schedule list|get|create|update|execute|delete|task-create|task-update|task-delete
ptero network list|create|notes|primary|delete
ptero subuser list|get|create|update|delete
ptero backup list|get|create|download|lock|restore|delete

ptero terminal attach

ptero app user list|get|external|create|update|delete
ptero app node list|get|deployable|configuration|create|update|delete
                 |allocations|allocation-create|allocation-delete
ptero app location list|get|create|update|delete
ptero app server list|get|external|create|details|build|startup
                   |suspend|unsuspend|reinstall|delete
                   |db-list|db-get|db-create|db-reset|db-delete
ptero app nest list|get
ptero app egg list|get

ptero remote sftp-auth|servers|servers-reset|activity|server
                     |install|install-complete
                     |transfer-failure|transfer-success
                     |backup-upload|backup-status|backup-restore
```

## Global flags / env

| Flag | Env | Notes |
|------|-----|-------|
| `--url` | `PTERO_URL` | Panel base URL |
| `--client-key` | `PTERO_CLIENT_KEY` | Client API |
| `--app-key` | `PTERO_APP_KEY` | Application API |
| `--daemon-token` | `PTERO_DAEMON_TOKEN` | Remote |
| `--json` | — | JSON stdout |
| `--config` | — | Config path |

## Examples

### Client

```bash
ptero --url https://panel.example.com --client-key ptlc_... server list --json
ptero server get <id>
ptero server resources <id> --json
ptero server power <id> start|stop|restart|kill
ptero server command <id> "list"
ptero server rename <id> "New Name" --description "…"
ptero server startup <id> --json
ptero server startup-var <id> SERVER_JARFILE server.jar

ptero file list <id> /
ptero file contents <id> /eula.txt
ptero file write <id> /eula.txt --contents "eula=true"
ptero file write <id> /cfg.toml --from-file ./cfg.toml
ptero file mkdir <id> / plugins
ptero file delete <id> / --files "old.jar"
ptero file compress <id> / --files "world"
ptero file decompress <id> / world.zip
ptero file pull <id> https://example.com/a.jar --directory /plugins --filename a.jar
ptero file upload-url <id>

ptero db list <id> --json
ptero schedule list <id> --json
ptero network list <id> --json
ptero subuser list <id> --json
ptero backup list <id> --json
ptero backup create <id> --name nightly
ptero backup restore <id> <backup_uuid> --truncate

ptero terminal attach <id>
ptero account get --json
```

Server id：`server list` 返回的短 id 或 UUID（Client 路由均可）。

### Application

```bash
ptero app user list --json
ptero app user create user@example.com alice Alice Example --password '…'
ptero app node list --json
ptero app node configuration <node_id> --json
ptero app location list --json
ptero app nest list --json
ptero app egg list <nest_id> --json
ptero app server list --json
ptero app server create --json-body '{"name":"demo","user":1,"egg":1,…}'
ptero app server suspend <id>
ptero app server unsuspend <id>
ptero app server delete <id> --force
```

`create` / `update` / `details` / `build` / `startup` 的 body 跟 Panel **Application API** schema；从 Panel 文档或已知良好的 `--json` GET 复制字段，不要臆造。

### Remote

```bash
ptero remote servers
ptero remote server <uuid>
ptero remote install <uuid>
ptero remote install-complete <uuid> --successful true
ptero remote sftp-auth <username> <password>
```

需要 node daemon token（`id.secret`），不是 `ptlc_` / `ptla_`。
