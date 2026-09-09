# Auth surfaces（产品）

Skill 为 execute-first 操作员代跑；本文件是鉴权/config 备查，不是替代 shell 执行。

`ptero` 对应 Panel 三个 API 前缀，必须用匹配凭证：

| Surface | Path | Credential | Header |
|---------|------|------------|--------|
| **Client** | `/api/client` | `ptlc_…` | `Authorization: Bearer <key>` |
| **Application** | `/api/application` | `ptla_…` | `Authorization: Bearer <key>` |
| **Remote** | `/api/remote` | node daemon `id.secret` | `Authorization: Bearer <id>.<secret>` |

Accept（客户端已处理）：`Application/vnd.pterodactyl.v1+json`。

## 命令 ↔ 密钥

| Need | Credential | Top-level |
|------|------------|-----------|
| 自己的账户 / 可访问服务器 / 文件 / 控制台 | Client `ptlc_` | `account` `server` `file` `db` `schedule` `network` `subuser` `backup` `terminal` |
| 面板管理：用户、节点、建服、nests/eggs | Application `ptla_` | `app …` |
| Wings/node daemon Remote | Daemon `id.secret` | `remote …` |

三套可同时写入 config；按命令族只发送对应那一套。

## 密钥来源

- **Client / Application**：Panel → Account（或 Admin）→ API Credentials。
- **Daemon token**：节点 / Wings 配置（`token_id` + `token` → `id.secret`）。不是 `ptl*` 用户 key。

## 推荐写入

```bash
ptero config set-url https://panel.example.com
ptero config set-client-key ptlc_...
ptero config set-app-key ptla_...           # 可选
ptero config set-daemon-token id.secret     # 可选
ptero config show
```

One-shot（尽量少把密钥留在 shell history）：

```bash
ptero --url https://panel.example.com --client-key KEY server list --json
```

Env：`PTERO_URL` `PTERO_CLIENT_KEY` `PTERO_APP_KEY` `PTERO_DAEMON_TOKEN`（仍视为敏感）。

## 操作提示

- TUI Help（`5`）只显示 Client/App/Remote **是否已配置**，不显示密文。
- Websocket 控制台：Client API 返回 `{ data: { token, socket } }`；`ptero terminal attach` 已封装该路径。
- 翼龙等中国 Panel：同一三面模型；`--url` 用该面板 HTTPS origin。
