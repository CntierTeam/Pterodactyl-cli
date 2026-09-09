---
name: pterodactyl-cli
description: >-
  Operate Pterodactyl / 翼龙 Panel via the `ptero` binary by running it for the
  user (TUI with no args, CLI with subcommands). Covers install, config keys
  (Client / Application / Remote), common panel workflows (servers, power,
  files, terminal, app admin), and TUI keys. Prefer shell execution over pasting
  recipes. Trigger on: ptero, Pterodactyl-Cli, Pterodactyl Panel, 翼龙, Wings
  console, client/app/remote API keys, panel server list/power/files/terminal.
license: GPL-3.0-only
metadata:
  short-description: 代跑 ptero（面板服务器/文件/终端）
---

# Pterodactyl-Cli (`ptero`)

产品：**`ptero`** — [Pterodactyl Panel](https://github.com/pterodactyl/panel)（含 [翼龙](https://github.com/pterodactyl-china/panel)）的 CLI/TUI 客户端。

你是 **操作员**：用户要装客户端、配面板、列服务器、开关机、管文件、挂 Wings 控制台、跑 Application/Remote → **自己在 shell 执行 `ptero`**，不要只拼命令给用户。

本 skill 是 **execute-first**：代跑产品，不是开发 Rust crates。细节命令树见 [references/cli.md](references/cli.md)；鉴权见 [references/auth.md](references/auth.md)。

Repo: https://github.com/CntierTeam/Pterodactyl-cli

## Agent 硬规则

1. **执行优先**：能跑就跑。二进制：`ptero` 或 `~/.local/bin/ptero`；没有就先装（见下方 Install）。
2. **禁止**用「组装指令 / 操作手册 / SAMPLE / YOUR_CLI / 长篇理科说明」代替执行。短句说明 → 立刻跑 → 根据输出继续。
3. **无参数 → TUI**；**有子命令 → CLI**。Agent 代控优先 **CLI + `--json`**；用户明确要全屏 UI 再开 `ptero`。
4. 命令名永远 **`ptero`**，禁止 `SAMPLE` / `YOUR_CLI`。缺 URL 或对应密钥时只问缺的那一项，问完继续跑。
5. **不要发明 flags/endpoints**。不确定就跑 `ptero <cmd> --help`，以输出和 [references/cli.md](references/cli.md) 为准。
6. **选对鉴权面**：Client `ptlc_…` / Application `ptla_…` / Remote `id.secret`。错面 → 401/403。见 [references/auth.md](references/auth.md)。
7. **Secrets**：不回显完整 API key / daemon token；优先 `ptero config set-*` 一次写入。破坏性操作（`power kill`、`reinstall`、`delete`、`backup restore`、app server delete）意图不清时先确认。
8. 真机意图就真跑 Panel；不要默认改成「只能帮你写命令」。

## 标准代跑流

```bash
command -v ptero || ~/.local/bin/ptero --help
ptero config show || true
ptero server list --json
```

未配置则先（按需写入对应密钥，摘要里 redact）：

```bash
ptero config set-url 'https://panel.example.com'
ptero config set-client-key 'ptlc_...'      # 多数用户工作流
ptero config set-app-key 'ptla_...'         # 管理员 Application
ptero config set-daemon-token 'id.secret'   # Remote / Wings node
ptero config show
```

## 意图 → 怎么跑

| 用户意图 | 执行 |
|----------|------|
| 装 / 升级客户端 | 跑下方 install，再 `ptero --help` |
| 配 URL / 密钥 | `config set-url` / `set-client-key` / `set-app-key` / `set-daemon-token` / `show` |
| 列服务器 / 详情 / 资源 | `server list\|get\|resources --json` |
| 开 / 停 / 重启 / 强杀 | `server power <id> start\|stop\|restart\|kill` |
| 发控制台命令 | `server command <id> "say hello"` |
| 挂 Wings 控制台 | `terminal attach <id>`（或 TUI → Servers → Enter） |
| 文件 | `file list\|contents\|write\|mkdir\|compress\|pull …` |
| DB / 计划 / 网络 / 子用户 / 备份 | `db` / `schedule` / `network` / `subuser` / `backup` |
| 账户 / API keys | `account get\|api-keys\|activity --json` |
| Application 管理 | `app user\|node\|location\|server\|nest\|egg …`（要 `ptla_`） |
| Remote / daemon | `remote servers\|server\|install …`（要 `id.secret`） |
| 要 TUI | 启动无参 `ptero`，并告知键位 |

## Install（仅当本机没有 ptero）

Prebuilts: [Releases](https://github.com/CntierTeam/Pterodactyl-cli/releases)（`main` → Continuous；`v*` → 正式版）。

```bash
curl -fsSL https://raw.githubusercontent.com/CntierTeam/Pterodactyl-cli/main/scripts/install.sh | bash
# Continuous / pin:
# bash -s -- --continuous
# bash -s -- --version v0.1.0
command -v ptero && ptero --help
```

Windows（PowerShell）：`irm …/install.ps1 | iex`（asset: **windows-amd64**）。

## Config 速查

默认：`~/.config/ptero/config.toml`（`ptero config path`）。

| Flag | Env | 用途 |
|------|-----|------|
| `--url` | `PTERO_URL` | Panel base URL |
| `--client-key` | `PTERO_CLIENT_KEY` | `/api/client`（`ptlc_…`） |
| `--app-key` | `PTERO_APP_KEY` | `/api/application`（`ptla_…`） |
| `--daemon-token` | `PTERO_DAEMON_TOKEN` | `/api/remote`（`id.secret`） |
| `--json` | — | JSON 输出 |
| `--config` | — | 另一份 config |

One-shot：`ptero --url https://panel.example.com --client-key ptlc_... --json server list`

## Auth 面（速查）

| 面 | 密钥形态 | 命令族 |
|----|----------|--------|
| **Client** | `ptlc_…` | `account` `server` `file` `db` `schedule` `network` `subuser` `backup` `terminal` |
| **Application** | `ptla_…` | `app user\|node\|location\|server\|nest\|egg` |
| **Remote** | `id.secret` | `remote …` |

细节：[references/auth.md](references/auth.md)。

## TUI 键位（用户自己玩时）

| Key | Action |
|-----|--------|
| `0` | Home |
| `1` | Servers（`j`/`k`；`o` start / `s` stop / `r` restart；Enter → terminal） |
| `2` | Account |
| `3` / `4` | App Users / App Nodes（要 app key） |
| `5` / `h` / `?` | Help / Config 状态 |
| `R` | Refresh（非 terminal） |
| Esc | 离开 terminal |
| `q` | Quit（非输入中） |

## Troubleshooting

| 症状 | 处理 |
|------|------|
| 未配置 / list 空 | `config set-url` + 对应 `set-*-key` |
| 401 / 403 | 检查是否用错 Client / App / Remote 面 |
| `terminal attach` 失败 | 确认 Client key + 服务器 id；先 `server websocket <id>` 看凭证是否返回 |
| `ptero` not found | 装二进制并把 `~/.local/bin` 加 PATH |

https://github.com/CntierTeam/Pterodactyl-cli
