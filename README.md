# Pterodactyl-Cli

面向 [Pterodactyl Panel](https://github.com/pterodactyl/panel) 的 Rust CLI / TUI 客户端（兼容 [pterodactyl-china/panel](https://github.com/pterodactyl-china/panel)）。

- **无参数** → 启动 TUI
- **带子命令** → 走 CLI
- 协议约定见 [`docs/CONTRACT.md`](docs/CONTRACT.md)
- 功能对齐清单见 [`docs/FEATURE_PARITY.md`](docs/FEATURE_PARITY.md)

二进制名：`ptero`。

许可证：仓库根目录 [`LICENSE`](LICENSE)（GNU GPLv3）。

预编译包见 [Releases](https://github.com/CntierTeam/Pterodactyl-cli/releases)：`main` 每次推送更新 **Continuous** 预发布；打 `v*` tag 发布正式版。

## 安装（从 Release 拉取）

一键安装最新**正式版**到 `~/.local/bin`：

```bash
curl -fsSL https://raw.githubusercontent.com/CntierTeam/Pterodactyl-cli/main/scripts/install.sh | bash
```

跟踪 `main` 的 Continuous 构建：

```bash
curl -fsSL https://raw.githubusercontent.com/CntierTeam/Pterodactyl-cli/main/scripts/install.sh | bash -s -- --continuous
```

指定版本 / 安装前缀：

```bash
curl -fsSL https://raw.githubusercontent.com/CntierTeam/Pterodactyl-cli/main/scripts/install.sh | bash -s -- --version v0.1.0
PREFIX=/usr/local curl -fsSL https://raw.githubusercontent.com/CntierTeam/Pterodactyl-cli/main/scripts/install.sh | bash
```

本地仓库内也可：`./scripts/install.sh [--continuous|--version vX.Y.Z]`。

### Windows（PowerShell）

默认安装到 `%LOCALAPPDATA%\Programs\ptero`，并写入用户 PATH：

```powershell
irm https://raw.githubusercontent.com/CntierTeam/Pterodactyl-cli/main/scripts/install.ps1 | iex
```

Continuous / 指定版本：

```powershell
& ([scriptblock]::Create((irm https://raw.githubusercontent.com/CntierTeam/Pterodactyl-cli/main/scripts/install.ps1))) -Continuous
& ([scriptblock]::Create((irm https://raw.githubusercontent.com/CntierTeam/Pterodactyl-cli/main/scripts/install.ps1))) -Version v0.1.0
```

本地：`.\scripts\install.ps1 [-Continuous|-Version vX.Y.Z]`。当前 Release 提供 **windows-amd64**（`ptero-windows-amd64.zip`）。

## 要求

- Rust 1.75+（见 [`rust-toolchain.toml`](rust-toolchain.toml)）
- 可访问的 Pterodactyl Panel
- 按需凭证：
  - **Client API key**（`ptlc_…`）→ `/api/client/*`
  - **Application API key**（`ptla_…`）→ `/api/application/*`
  - **Node daemon token**（`id.secret`）→ `/api/remote/*`

## 构建

```bash
cargo build --release -p ptero
./target/release/ptero --help
```

安装到 PATH（可选）：

```bash
cargo install --path bin/ptero
```

## 配置

配置文件默认：`~/.config/ptero/config.toml`

```bash
ptero config set-url https://panel.example.com
ptero config set-client-key ptlc_...
ptero config set-app-key ptla_...
ptero config set-daemon-token id.secret
ptero config show
```

也可用全局参数覆盖：`--url`、`--client-key`、`--app-key`、`--daemon-token`（或环境变量 `PTERO_URL` / `PTERO_CLIENT_KEY` / `PTERO_APP_KEY` / `PTERO_DAEMON_TOKEN`）。多数子命令支持 `--json`。

## 快速示例

```bash
ptero                          # TUI
ptero server list --json
ptero server power <id> start
ptero account get
ptero app user list
ptero remote servers
ptero terminal attach <id>
```

## 主要命令

| 命令 | 说明 |
|------|------|
| `config` | 本地客户端配置 |
| `account` | 账户 / 2FA / API Key / SSH Key / 活动 |
| `server` | 列表 / 资源 / 电源 / 启动项 / 设置 |
| `file` | 文件浏览、读写、压缩、拉取 |
| `db` | 数据库 |
| `schedule` | 计划任务 |
| `network` | 分配端口 |
| `subuser` | 子用户 |
| `backup` | 备份 |
| `terminal` | Wings 控制台附着 |
| `app` | Application API（user/node/location/server/nest/egg） |
| `remote` | Remote / Daemon API |

完整帮助：`ptero <command> --help`。

## TUI

| 键 | 界面 |
|----|------|
| `0` | Home |
| `1` | Servers（`o`/`s`/`r` 电源，Enter 终端） |
| `2` | Account |
| `3` | App Users |
| `4` | App Nodes |
| `5` | Help / Config |
| `q` | Quit |

## Workspace 结构

| Crate | 职责 |
|-------|------|
| `ptero-protocol` | Fractal DTO / 错误 / WS 常量 |
| `ptero-panel` | Panel HTTP（Client / Application / Remote） |
| `ptero-wings` | Wings 控制台 WebSocket |
| `ptero-config` | 本地配置读写 |
| `ptero-core` | 领域服务（CLI/TUI 唯一业务入口） |
| `ptero-cli` | Clap 子命令 |
| `ptero-tui` | Ratatui 界面 |
| `ptero` | 二进制入口（`bin/ptero`） |

硬规则：CLI / TUI **不得**直接发 HTTP/WS，一律经 `ptero-core`；跨 crate 类型只放在 `ptero-protocol`。

## Codex Skill

项目内 skill：[`skills/pterodactyl-cli/`](skills/pterodactyl-cli/)

```bash
./scripts/install-codex-skill.sh        # 复制到 ~/.codex/skills/pterodactyl-cli
./scripts/install-codex-skill.sh link   # 软链（改仓库即生效）
```

## 架构简述

```
ptero (bin)
├── 无 argv → ptero-tui
└── 有子命令 → ptero-cli
        └── ptero-core
              ├── ptero-panel  → Panel HTTP（三套 Bearer）
              └── ptero-wings  → Wings 控制台 WS
```

## 相关链接

- 上游面板：[pterodactyl/panel](https://github.com/pterodactyl/panel)
- 本仓库：[CntierTeam/Pterodactyl-cli](https://github.com/CntierTeam/Pterodactyl-cli)
