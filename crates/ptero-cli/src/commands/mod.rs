//! Clap command tree and dispatch for `ptero`.

mod dispatch;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

pub use dispatch::dispatch;

#[derive(Debug, Parser)]
#[command(name = "ptero", about = "Pterodactyl Panel CLI/TUI", version)]
pub struct Cli {
    #[arg(long, global = true, env = "PTERO_URL")]
    pub url: Option<String>,
    #[arg(long = "client-key", global = true, env = "PTERO_CLIENT_KEY")]
    pub client_key: Option<String>,
    #[arg(long = "app-key", global = true, env = "PTERO_APP_KEY")]
    pub app_key: Option<String>,
    #[arg(long = "daemon-token", global = true, env = "PTERO_DAEMON_TOKEN")]
    pub daemon_token: Option<String>,
    #[arg(long, global = true)]
    pub json: bool,
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    Config {
        #[command(subcommand)]
        action: ConfigCmd,
    },
    Account {
        #[command(subcommand)]
        action: AccountCmd,
    },
    Server {
        #[command(subcommand)]
        action: ServerCmd,
    },
    File {
        #[command(subcommand)]
        action: FileCmd,
    },
    Db {
        #[command(subcommand)]
        action: DbCmd,
    },
    Schedule {
        #[command(subcommand)]
        action: ScheduleCmd,
    },
    Network {
        #[command(subcommand)]
        action: NetworkCmd,
    },
    Subuser {
        #[command(subcommand)]
        action: SubuserCmd,
    },
    Backup {
        #[command(subcommand)]
        action: BackupCmd,
    },
    Terminal {
        #[command(subcommand)]
        action: TerminalCmd,
    },
    App {
        #[command(subcommand)]
        action: AppCmd,
    },
    Remote {
        #[command(subcommand)]
        action: RemoteCmd,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum ConfigCmd {
    Show,
    Path,
    SetUrl { url: String },
    SetClientKey { key: String },
    SetAppKey { key: String },
    SetDaemonToken { token: String },
}

#[derive(Debug, Clone, Subcommand)]
pub enum AccountCmd {
    Get,
    Email {
        email: String,
        password: String,
    },
    Password {
        current: String,
        new_password: String,
    },
    TwoFactor,
    TwoFactorEnable {
        code: String,
        password: String,
    },
    TwoFactorDisable {
        password: String,
    },
    Activity,
    ApiKeys,
    ApiKeyCreate {
        description: String,
        #[arg(long, default_value = "")]
        allowed_ips: String,
    },
    ApiKeyDelete {
        identifier: String,
    },
    SshKeys,
    SshKeyCreate {
        name: String,
        public_key: String,
    },
    SshKeyDelete {
        fingerprint: String,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum ServerCmd {
    List,
    Permissions,
    Get {
        server: String,
    },
    Resources {
        server: String,
    },
    Activity {
        server: String,
    },
    Power {
        server: String,
        signal: String,
    },
    Command {
        server: String,
        command: String,
    },
    Websocket {
        server: String,
    },
    Startup {
        server: String,
    },
    StartupVar {
        server: String,
        key: String,
        value: String,
    },
    StartupEgg {
        server: String,
        #[arg(long)]
        json_body: String,
    },
    Rename {
        server: String,
        name: String,
        #[arg(long)]
        description: Option<String>,
    },
    Reinstall {
        server: String,
    },
    DockerImage {
        server: String,
        docker_image: String,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum FileCmd {
    List {
        server: String,
        #[arg(default_value = "/")]
        directory: String,
    },
    Contents {
        server: String,
        file: String,
    },
    Download {
        server: String,
        file: String,
    },
    Write {
        server: String,
        file: String,
        #[arg(long)]
        contents: Option<String>,
        #[arg(long)]
        from_file: Option<PathBuf>,
    },
    Rename {
        server: String,
        root: String,
        from: String,
        to: String,
    },
    Copy {
        server: String,
        location: String,
    },
    Compress {
        server: String,
        root: String,
        #[arg(long)]
        files: String,
    },
    Decompress {
        server: String,
        root: String,
        file: String,
    },
    Delete {
        server: String,
        root: String,
        #[arg(long)]
        files: String,
    },
    Mkdir {
        server: String,
        root: String,
        name: String,
    },
    Chmod {
        server: String,
        root: String,
        file: String,
        mode: String,
    },
    Pull {
        server: String,
        url: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        filename: Option<String>,
    },
    UploadUrl {
        server: String,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum DbCmd {
    List {
        server: String,
    },
    Create {
        server: String,
        database: String,
        remote: String,
    },
    Rotate {
        server: String,
        database: String,
    },
    Delete {
        server: String,
        database: String,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum ScheduleCmd {
    List {
        server: String,
    },
    Get {
        server: String,
        schedule: u64,
    },
    Create {
        server: String,
        name: String,
        #[arg(long, default_value = "*/5")]
        minute: String,
        #[arg(long, default_value = "*")]
        hour: String,
        #[arg(long, default_value = "*")]
        day_of_month: String,
        #[arg(long, default_value = "*")]
        month: String,
        #[arg(long, default_value = "*")]
        day_of_week: String,
        #[arg(long, default_value_t = true)]
        active: bool,
    },
    Update {
        server: String,
        schedule: u64,
        #[arg(long)]
        json_body: String,
    },
    Execute {
        server: String,
        schedule: u64,
    },
    Delete {
        server: String,
        schedule: u64,
    },
    TaskCreate {
        server: String,
        schedule: u64,
        action: String,
        payload: String,
        #[arg(long, default_value_t = 0)]
        time_offset: u64,
    },
    TaskUpdate {
        server: String,
        schedule: u64,
        task: u64,
        #[arg(long)]
        json_body: String,
    },
    TaskDelete {
        server: String,
        schedule: u64,
        task: u64,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum NetworkCmd {
    List {
        server: String,
    },
    Create {
        server: String,
    },
    Notes {
        server: String,
        allocation: u64,
        notes: String,
    },
    Primary {
        server: String,
        allocation: u64,
    },
    Delete {
        server: String,
        allocation: u64,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum SubuserCmd {
    List {
        server: String,
    },
    Get {
        server: String,
        user: String,
    },
    Create {
        server: String,
        email: String,
        #[arg(long)]
        permissions: String,
    },
    Update {
        server: String,
        user: String,
        email: String,
        #[arg(long)]
        permissions: String,
    },
    Delete {
        server: String,
        user: String,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum BackupCmd {
    List {
        server: String,
    },
    Get {
        server: String,
        backup: String,
    },
    Create {
        server: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        ignored: Option<String>,
        #[arg(long)]
        locked: bool,
    },
    Download {
        server: String,
        backup: String,
    },
    Lock {
        server: String,
        backup: String,
    },
    Restore {
        server: String,
        backup: String,
        #[arg(long)]
        truncate: bool,
    },
    Delete {
        server: String,
        backup: String,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum TerminalCmd {
    Attach { server: String },
}

#[derive(Debug, Clone, Subcommand)]
pub enum AppCmd {
    User {
        #[command(subcommand)]
        action: AppUserCmd,
    },
    Node {
        #[command(subcommand)]
        action: AppNodeCmd,
    },
    Location {
        #[command(subcommand)]
        action: AppLocationCmd,
    },
    Server {
        #[command(subcommand)]
        action: AppServerCmd,
    },
    Nest {
        #[command(subcommand)]
        action: AppNestCmd,
    },
    Egg {
        #[command(subcommand)]
        action: AppEggCmd,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum AppUserCmd {
    List,
    Get {
        id: u64,
    },
    External {
        external_id: String,
    },
    Create {
        email: String,
        username: String,
        first_name: String,
        last_name: String,
        #[arg(long)]
        password: Option<String>,
        #[arg(long)]
        root_admin: bool,
    },
    Update {
        id: u64,
        #[arg(long)]
        json_body: String,
    },
    Delete {
        id: u64,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum AppNodeCmd {
    List,
    Get {
        id: u64,
    },
    Deployable,
    Configuration {
        id: u64,
    },
    Create {
        #[arg(long)]
        json_body: String,
    },
    Update {
        id: u64,
        #[arg(long)]
        json_body: String,
    },
    Delete {
        id: u64,
    },
    Allocations {
        node: u64,
    },
    AllocationCreate {
        node: u64,
        #[arg(long)]
        json_body: String,
    },
    AllocationDelete {
        node: u64,
        allocation: u64,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum AppLocationCmd {
    List,
    Get {
        id: u64,
    },
    Create {
        short: String,
        long: String,
    },
    Update {
        id: u64,
        #[arg(long)]
        json_body: String,
    },
    Delete {
        id: u64,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum AppServerCmd {
    List,
    Get {
        id: u64,
    },
    External {
        external_id: String,
    },
    Create {
        #[arg(long)]
        json_body: String,
    },
    Details {
        id: u64,
        #[arg(long)]
        json_body: String,
    },
    Build {
        id: u64,
        #[arg(long)]
        json_body: String,
    },
    Startup {
        id: u64,
        #[arg(long)]
        json_body: String,
    },
    Suspend {
        id: u64,
    },
    Unsuspend {
        id: u64,
    },
    Reinstall {
        id: u64,
    },
    Delete {
        id: u64,
        #[arg(long)]
        force: bool,
    },
    DbList {
        server: u64,
    },
    DbGet {
        server: u64,
        database: u64,
    },
    DbCreate {
        server: u64,
        #[arg(long)]
        json_body: String,
    },
    DbReset {
        server: u64,
        database: u64,
    },
    DbDelete {
        server: u64,
        database: u64,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum AppNestCmd {
    List,
    Get { id: u64 },
}

#[derive(Debug, Clone, Subcommand)]
pub enum AppEggCmd {
    List { nest: u64 },
    Get { nest: u64, egg: u64 },
}

#[derive(Debug, Clone, Subcommand)]
pub enum RemoteCmd {
    SftpAuth {
        username: String,
        password: String,
    },
    Servers,
    ServersReset,
    Activity {
        #[arg(long)]
        json_body: String,
    },
    Server {
        uuid: String,
    },
    Install {
        uuid: String,
    },
    InstallComplete {
        uuid: String,
        #[arg(long, default_value_t = true)]
        successful: bool,
    },
    TransferFailure {
        uuid: String,
    },
    TransferSuccess {
        uuid: String,
    },
    BackupUpload {
        backup: String,
    },
    BackupStatus {
        backup: String,
        #[arg(long)]
        json_body: String,
    },
    BackupRestore {
        backup: String,
        #[arg(long, default_value_t = true)]
        successful: bool,
    },
}
