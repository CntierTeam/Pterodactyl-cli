//! Panel API traits covering Client, Application, and Remote surfaces.

use async_trait::async_trait;
use ptero_protocol::account::*;
use ptero_protocol::application::*;
use ptero_protocol::backup::*;
use ptero_protocol::files::*;
use ptero_protocol::network::*;
use ptero_protocol::remote::*;
use ptero_protocol::schedule::*;
use ptero_protocol::server::*;
use ptero_protocol::{FractalItem, FractalList, PteroResult, WebsocketCredentials};
use serde_json::Value;

/// Client API (`/api/client/*`) — Bearer client API key.
#[async_trait]
pub trait ClientApi: Send + Sync {
    async fn raw(
        &self,
        method: &str,
        path: &str,
        query: &[(String, String)],
        body: Option<&Value>,
    ) -> PteroResult<Value>;

    // --- Root ---
    async fn list_servers(
        &self,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<ServerAttributes>>;
    async fn permissions(&self) -> PteroResult<Value>;

    // --- Account ---
    async fn account(&self) -> PteroResult<FractalItem<AccountAttributes>>;
    async fn update_email(&self, req: &UpdateEmailRequest) -> PteroResult<()>;
    async fn update_password(&self, req: &UpdatePasswordRequest) -> PteroResult<()>;
    async fn two_factor_details(&self) -> PteroResult<Value>;
    async fn enable_two_factor(&self, req: &EnableTwoFactorRequest) -> PteroResult<Value>;
    async fn disable_two_factor(&self, req: &DisableTwoFactorRequest) -> PteroResult<()>;
    async fn account_activity(
        &self,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<ActivityAttributes>>;
    async fn list_api_keys(&self) -> PteroResult<FractalList<ApiKeyAttributes>>;
    async fn create_api_key(
        &self,
        req: &CreateApiKeyRequest,
    ) -> PteroResult<FractalItem<ApiKeyAttributes>>;
    async fn delete_api_key(&self, identifier: &str) -> PteroResult<()>;
    async fn list_ssh_keys(&self) -> PteroResult<FractalList<SshKeyAttributes>>;
    async fn create_ssh_key(
        &self,
        req: &CreateSshKeyRequest,
    ) -> PteroResult<FractalItem<SshKeyAttributes>>;
    async fn delete_ssh_key(&self, req: &DeleteSshKeyRequest) -> PteroResult<()>;

    // --- Server core ---
    async fn get_server(&self, server: &str) -> PteroResult<FractalItem<ServerAttributes>>;
    async fn server_websocket(&self, server: &str) -> PteroResult<WebsocketCredentials>;
    async fn server_resources(&self, server: &str) -> PteroResult<FractalItem<ResourceStats>>;
    async fn server_activity(
        &self,
        server: &str,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<ActivityAttributes>>;
    async fn server_command(&self, server: &str, command: &str) -> PteroResult<()>;
    async fn server_power(&self, server: &str, signal: PowerSignal) -> PteroResult<()>;

    // --- Databases ---
    async fn list_databases(&self, server: &str) -> PteroResult<FractalList<DatabaseAttributes>>;
    async fn create_database(
        &self,
        server: &str,
        req: &CreateDatabaseRequest,
    ) -> PteroResult<FractalItem<DatabaseAttributes>>;
    async fn rotate_database_password(
        &self,
        server: &str,
        database: &str,
    ) -> PteroResult<FractalItem<DatabaseAttributes>>;
    async fn delete_database(&self, server: &str, database: &str) -> PteroResult<()>;

    // --- Files ---
    async fn list_files(
        &self,
        server: &str,
        directory: &str,
    ) -> PteroResult<FractalList<FileEntry>>;
    async fn file_contents(&self, server: &str, file: &str) -> PteroResult<String>;
    async fn file_download(
        &self,
        server: &str,
        file: &str,
    ) -> PteroResult<FractalItem<SignedUrlAttributes>>;
    async fn file_rename(&self, server: &str, req: &RenameFileRequest) -> PteroResult<()>;
    async fn file_copy(&self, server: &str, req: &CopyFileRequest) -> PteroResult<()>;
    async fn file_write(&self, server: &str, file: &str, contents: &str) -> PteroResult<()>;
    async fn file_compress(
        &self,
        server: &str,
        req: &CompressFilesRequest,
    ) -> PteroResult<FractalItem<FileEntry>>;
    async fn file_decompress(&self, server: &str, req: &DecompressFileRequest) -> PteroResult<()>;
    async fn file_delete(&self, server: &str, req: &DeleteFilesRequest) -> PteroResult<()>;
    async fn file_create_folder(&self, server: &str, req: &CreateFolderRequest) -> PteroResult<()>;
    async fn file_chmod(&self, server: &str, req: &ChmodFilesRequest) -> PteroResult<()>;
    async fn file_pull(&self, server: &str, req: &PullFileRequest) -> PteroResult<()>;
    async fn file_upload_url(&self, server: &str) -> PteroResult<FractalItem<SignedUrlAttributes>>;

    // --- Schedules ---
    async fn list_schedules(&self, server: &str) -> PteroResult<FractalList<ScheduleAttributes>>;
    async fn create_schedule(
        &self,
        server: &str,
        req: &CreateScheduleRequest,
    ) -> PteroResult<FractalItem<ScheduleAttributes>>;
    async fn get_schedule(
        &self,
        server: &str,
        schedule: u64,
    ) -> PteroResult<FractalItem<ScheduleAttributes>>;
    async fn update_schedule(
        &self,
        server: &str,
        schedule: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<ScheduleAttributes>>;
    async fn execute_schedule(&self, server: &str, schedule: u64) -> PteroResult<()>;
    async fn delete_schedule(&self, server: &str, schedule: u64) -> PteroResult<()>;
    async fn create_schedule_task(
        &self,
        server: &str,
        schedule: u64,
        req: &CreateTaskRequest,
    ) -> PteroResult<FractalItem<TaskAttributes>>;
    async fn update_schedule_task(
        &self,
        server: &str,
        schedule: u64,
        task: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<TaskAttributes>>;
    async fn delete_schedule_task(&self, server: &str, schedule: u64, task: u64)
        -> PteroResult<()>;

    // --- Network ---
    async fn list_allocations(
        &self,
        server: &str,
    ) -> PteroResult<FractalList<AllocationAttributes>>;
    async fn create_allocation(
        &self,
        server: &str,
    ) -> PteroResult<FractalItem<AllocationAttributes>>;
    async fn update_allocation(
        &self,
        server: &str,
        allocation: u64,
        req: &UpdateAllocationRequest,
    ) -> PteroResult<FractalItem<AllocationAttributes>>;
    async fn set_primary_allocation(&self, server: &str, allocation: u64) -> PteroResult<()>;
    async fn delete_allocation(&self, server: &str, allocation: u64) -> PteroResult<()>;

    // --- Subusers ---
    async fn list_subusers(&self, server: &str) -> PteroResult<FractalList<SubuserAttributes>>;
    async fn create_subuser(
        &self,
        server: &str,
        req: &SubuserRequest,
    ) -> PteroResult<FractalItem<SubuserAttributes>>;
    async fn get_subuser(
        &self,
        server: &str,
        user: &str,
    ) -> PteroResult<FractalItem<SubuserAttributes>>;
    async fn update_subuser(
        &self,
        server: &str,
        user: &str,
        req: &SubuserRequest,
    ) -> PteroResult<FractalItem<SubuserAttributes>>;
    async fn delete_subuser(&self, server: &str, user: &str) -> PteroResult<()>;

    // --- Backups ---
    async fn list_backups(&self, server: &str) -> PteroResult<FractalList<BackupAttributes>>;
    async fn create_backup(
        &self,
        server: &str,
        req: &CreateBackupRequest,
    ) -> PteroResult<FractalItem<BackupAttributes>>;
    async fn get_backup(
        &self,
        server: &str,
        backup: &str,
    ) -> PteroResult<FractalItem<BackupAttributes>>;
    async fn backup_download(
        &self,
        server: &str,
        backup: &str,
    ) -> PteroResult<FractalItem<SignedUrlAttributes>>;
    async fn toggle_backup_lock(
        &self,
        server: &str,
        backup: &str,
    ) -> PteroResult<FractalItem<BackupAttributes>>;
    async fn restore_backup(
        &self,
        server: &str,
        backup: &str,
        req: &RestoreBackupRequest,
    ) -> PteroResult<()>;
    async fn delete_backup(&self, server: &str, backup: &str) -> PteroResult<()>;

    // --- Startup / settings ---
    async fn startup(&self, server: &str) -> PteroResult<Value>;
    async fn update_startup_variable(
        &self,
        server: &str,
        req: &UpdateStartupVariableRequest,
    ) -> PteroResult<Value>;
    async fn update_startup_egg(&self, server: &str, body: &Value) -> PteroResult<Value>;
    async fn rename_server(&self, server: &str, req: &RenameServerRequest) -> PteroResult<()>;
    async fn reinstall_server(&self, server: &str) -> PteroResult<()>;
    async fn set_docker_image(&self, server: &str, req: &DockerImageRequest) -> PteroResult<()>;
}

/// Application API (`/api/application/*`) — Bearer application API key.
#[async_trait]
pub trait ApplicationApi: Send + Sync {
    async fn raw(
        &self,
        method: &str,
        path: &str,
        query: &[(String, String)],
        body: Option<&Value>,
    ) -> PteroResult<Value>;

    // Users
    async fn list_users(
        &self,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<AppUserAttributes>>;
    async fn get_user(&self, id: u64) -> PteroResult<FractalItem<AppUserAttributes>>;
    async fn get_user_external(
        &self,
        external_id: &str,
    ) -> PteroResult<FractalItem<AppUserAttributes>>;
    async fn create_user(
        &self,
        req: &CreateAppUserRequest,
    ) -> PteroResult<FractalItem<AppUserAttributes>>;
    async fn update_user(
        &self,
        id: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<AppUserAttributes>>;
    async fn delete_user(&self, id: u64) -> PteroResult<()>;

    // Nodes
    async fn list_nodes(
        &self,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<AppNodeAttributes>>;
    async fn deployable_nodes(&self, query: &[(String, String)]) -> PteroResult<Value>;
    async fn get_node(&self, id: u64) -> PteroResult<FractalItem<AppNodeAttributes>>;
    async fn node_configuration(&self, id: u64) -> PteroResult<Value>;
    async fn create_node(&self, body: &Value) -> PteroResult<FractalItem<AppNodeAttributes>>;
    async fn update_node(
        &self,
        id: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<AppNodeAttributes>>;
    async fn delete_node(&self, id: u64) -> PteroResult<()>;
    async fn list_node_allocations(
        &self,
        node: u64,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<AppAllocationAttributes>>;
    async fn create_node_allocations(&self, node: u64, body: &Value) -> PteroResult<Value>;
    async fn delete_node_allocation(&self, node: u64, allocation: u64) -> PteroResult<()>;

    // Locations
    async fn list_locations(
        &self,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<AppLocationAttributes>>;
    async fn get_location(&self, id: u64) -> PteroResult<FractalItem<AppLocationAttributes>>;
    async fn create_location(
        &self,
        req: &CreateLocationRequest,
    ) -> PteroResult<FractalItem<AppLocationAttributes>>;
    async fn update_location(
        &self,
        id: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<AppLocationAttributes>>;
    async fn delete_location(&self, id: u64) -> PteroResult<()>;

    // Servers
    async fn list_app_servers(
        &self,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<AppServerAttributes>>;
    async fn get_app_server(&self, id: u64) -> PteroResult<FractalItem<AppServerAttributes>>;
    async fn get_app_server_external(
        &self,
        external_id: &str,
    ) -> PteroResult<FractalItem<AppServerAttributes>>;
    async fn create_app_server(
        &self,
        body: &Value,
    ) -> PteroResult<FractalItem<AppServerAttributes>>;
    async fn update_app_server_details(
        &self,
        id: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<AppServerAttributes>>;
    async fn update_app_server_build(
        &self,
        id: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<AppServerAttributes>>;
    async fn update_app_server_startup(
        &self,
        id: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<AppServerAttributes>>;
    async fn suspend_app_server(&self, id: u64) -> PteroResult<()>;
    async fn unsuspend_app_server(&self, id: u64) -> PteroResult<()>;
    async fn reinstall_app_server(&self, id: u64) -> PteroResult<()>;
    async fn delete_app_server(&self, id: u64, force: bool) -> PteroResult<()>;

    // App server databases
    async fn list_app_databases(
        &self,
        server: u64,
    ) -> PteroResult<FractalList<AppDatabaseAttributes>>;
    async fn get_app_database(
        &self,
        server: u64,
        database: u64,
    ) -> PteroResult<FractalItem<AppDatabaseAttributes>>;
    async fn create_app_database(
        &self,
        server: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<AppDatabaseAttributes>>;
    async fn reset_app_database_password(&self, server: u64, database: u64) -> PteroResult<()>;
    async fn delete_app_database(&self, server: u64, database: u64) -> PteroResult<()>;

    // Nests / eggs
    async fn list_nests(
        &self,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<AppNestAttributes>>;
    async fn get_nest(&self, id: u64) -> PteroResult<FractalItem<AppNestAttributes>>;
    async fn list_eggs(
        &self,
        nest: u64,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<AppEggAttributes>>;
    async fn get_egg(&self, nest: u64, egg: u64) -> PteroResult<FractalItem<AppEggAttributes>>;
}

/// Remote API (`/api/remote/*`) — Bearer `daemon_token_id.daemon_token`.
#[async_trait]
pub trait RemoteApi: Send + Sync {
    async fn raw(
        &self,
        method: &str,
        path: &str,
        query: &[(String, String)],
        body: Option<&Value>,
    ) -> PteroResult<Value>;

    async fn sftp_auth(&self, req: &SftpAuthRequest) -> PteroResult<SftpAuthResponse>;
    async fn list_remote_servers(&self, query: &[(String, String)]) -> PteroResult<Value>;
    async fn reset_remote_servers(&self) -> PteroResult<()>;
    async fn ingest_activity(&self, body: &Value) -> PteroResult<()>;
    async fn remote_server_details(&self, uuid: &str) -> PteroResult<Value>;
    async fn remote_install_details(&self, uuid: &str) -> PteroResult<RemoteInstallDetails>;
    async fn remote_install_complete(
        &self,
        uuid: &str,
        req: &RemoteInstallCompleteRequest,
    ) -> PteroResult<()>;
    async fn remote_transfer_failure(&self, uuid: &str) -> PteroResult<()>;
    async fn remote_transfer_success(&self, uuid: &str) -> PteroResult<()>;
    async fn remote_backup_upload(&self, backup: &str) -> PteroResult<Value>;
    async fn remote_backup_status(
        &self,
        backup: &str,
        req: &RemoteBackupStatusRequest,
    ) -> PteroResult<()>;
    async fn remote_backup_restore(
        &self,
        backup: &str,
        req: &RemoteBackupRestoreRequest,
    ) -> PteroResult<()>;
}
