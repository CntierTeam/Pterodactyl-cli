//! Domain services shared by CLI and TUI. All HTTP/WS goes through panel/wings.

use crate::AppContext;
use ptero_panel::{ApplicationApi, ClientApi, RemoteApi};
use ptero_protocol::account::*;
use ptero_protocol::application::*;
use ptero_protocol::backup::*;
use ptero_protocol::files::*;
use ptero_protocol::network::*;
use ptero_protocol::remote::*;
use ptero_protocol::schedule::*;
use ptero_protocol::server::*;
use ptero_protocol::{FractalItem, FractalList, PteroResult, WebsocketCredentials};
use ptero_wings::{ConsoleEvent, TokenRefresh, WingsConsole};
use serde_json::Value;
use std::sync::Arc;

pub struct AccountService<'a>(pub &'a AppContext);
pub struct ServerService<'a>(pub &'a AppContext);
pub struct FileService<'a>(pub &'a AppContext);
pub struct DatabaseService<'a>(pub &'a AppContext);
pub struct ScheduleService<'a>(pub &'a AppContext);
pub struct NetworkService<'a>(pub &'a AppContext);
pub struct SubuserService<'a>(pub &'a AppContext);
pub struct BackupService<'a>(pub &'a AppContext);
pub struct ApplicationService<'a>(pub &'a AppContext);
pub struct RemoteService<'a>(pub &'a AppContext);
pub struct TerminalService<'a>(pub &'a AppContext);

impl AccountService<'_> {
    pub async fn get(&self) -> PteroResult<FractalItem<AccountAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.account().await
    }
    pub async fn update_email(&self, req: &UpdateEmailRequest) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.update_email(req).await
    }
    pub async fn update_password(&self, req: &UpdatePasswordRequest) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.update_password(req).await
    }
    pub async fn two_factor_details(&self) -> PteroResult<Value> {
        self.0.require_client_ready()?;
        self.0.panel.two_factor_details().await
    }
    pub async fn enable_two_factor(&self, req: &EnableTwoFactorRequest) -> PteroResult<Value> {
        self.0.require_client_ready()?;
        self.0.panel.enable_two_factor(req).await
    }
    pub async fn disable_two_factor(&self, req: &DisableTwoFactorRequest) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.disable_two_factor(req).await
    }
    pub async fn activity(
        &self,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<ActivityAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.account_activity(query).await
    }
    pub async fn list_api_keys(&self) -> PteroResult<FractalList<ApiKeyAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.list_api_keys().await
    }
    pub async fn create_api_key(
        &self,
        req: &CreateApiKeyRequest,
    ) -> PteroResult<FractalItem<ApiKeyAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.create_api_key(req).await
    }
    pub async fn delete_api_key(&self, identifier: &str) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.delete_api_key(identifier).await
    }
    pub async fn list_ssh_keys(&self) -> PteroResult<FractalList<SshKeyAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.list_ssh_keys().await
    }
    pub async fn create_ssh_key(
        &self,
        req: &CreateSshKeyRequest,
    ) -> PteroResult<FractalItem<SshKeyAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.create_ssh_key(req).await
    }
    pub async fn delete_ssh_key(&self, req: &DeleteSshKeyRequest) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.delete_ssh_key(req).await
    }
}

impl ServerService<'_> {
    pub async fn list(
        &self,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<ServerAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.list_servers(query).await
    }
    pub async fn permissions(&self) -> PteroResult<Value> {
        self.0.require_client_ready()?;
        self.0.panel.permissions().await
    }
    pub async fn get(&self, server: &str) -> PteroResult<FractalItem<ServerAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.get_server(server).await
    }
    pub async fn resources(&self, server: &str) -> PteroResult<FractalItem<ResourceStats>> {
        self.0.require_client_ready()?;
        self.0.panel.server_resources(server).await
    }
    pub async fn activity(
        &self,
        server: &str,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<ActivityAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.server_activity(server, query).await
    }
    pub async fn command(&self, server: &str, command: &str) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.server_command(server, command).await
    }
    pub async fn power(&self, server: &str, signal: PowerSignal) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.server_power(server, signal).await
    }
    pub async fn websocket(&self, server: &str) -> PteroResult<WebsocketCredentials> {
        self.0.require_client_ready()?;
        self.0.panel.server_websocket(server).await
    }
    pub async fn startup(&self, server: &str) -> PteroResult<Value> {
        self.0.require_client_ready()?;
        self.0.panel.startup(server).await
    }
    pub async fn update_startup_variable(
        &self,
        server: &str,
        req: &UpdateStartupVariableRequest,
    ) -> PteroResult<Value> {
        self.0.require_client_ready()?;
        self.0.panel.update_startup_variable(server, req).await
    }
    pub async fn update_startup_egg(&self, server: &str, body: &Value) -> PteroResult<Value> {
        self.0.require_client_ready()?;
        self.0.panel.update_startup_egg(server, body).await
    }
    pub async fn rename(&self, server: &str, req: &RenameServerRequest) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.rename_server(server, req).await
    }
    pub async fn reinstall(&self, server: &str) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.reinstall_server(server).await
    }
    pub async fn set_docker_image(
        &self,
        server: &str,
        req: &DockerImageRequest,
    ) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.set_docker_image(server, req).await
    }
}

impl FileService<'_> {
    pub async fn list(&self, server: &str, directory: &str) -> PteroResult<FractalList<FileEntry>> {
        self.0.require_client_ready()?;
        self.0.panel.list_files(server, directory).await
    }
    pub async fn contents(&self, server: &str, file: &str) -> PteroResult<String> {
        self.0.require_client_ready()?;
        self.0.panel.file_contents(server, file).await
    }
    pub async fn download(
        &self,
        server: &str,
        file: &str,
    ) -> PteroResult<FractalItem<ptero_protocol::files::SignedUrlAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.file_download(server, file).await
    }
    pub async fn rename(&self, server: &str, req: &RenameFileRequest) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.file_rename(server, req).await
    }
    pub async fn copy(&self, server: &str, req: &CopyFileRequest) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.file_copy(server, req).await
    }
    pub async fn write(&self, server: &str, file: &str, contents: &str) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.file_write(server, file, contents).await
    }
    pub async fn compress(
        &self,
        server: &str,
        req: &CompressFilesRequest,
    ) -> PteroResult<FractalItem<FileEntry>> {
        self.0.require_client_ready()?;
        self.0.panel.file_compress(server, req).await
    }
    pub async fn decompress(&self, server: &str, req: &DecompressFileRequest) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.file_decompress(server, req).await
    }
    pub async fn delete(&self, server: &str, req: &DeleteFilesRequest) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.file_delete(server, req).await
    }
    pub async fn create_folder(&self, server: &str, req: &CreateFolderRequest) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.file_create_folder(server, req).await
    }
    pub async fn chmod(&self, server: &str, req: &ChmodFilesRequest) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.file_chmod(server, req).await
    }
    pub async fn pull(&self, server: &str, req: &PullFileRequest) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.file_pull(server, req).await
    }
    pub async fn upload_url(
        &self,
        server: &str,
    ) -> PteroResult<FractalItem<ptero_protocol::files::SignedUrlAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.file_upload_url(server).await
    }
}

impl DatabaseService<'_> {
    pub async fn list(&self, server: &str) -> PteroResult<FractalList<DatabaseAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.list_databases(server).await
    }
    pub async fn create(
        &self,
        server: &str,
        req: &CreateDatabaseRequest,
    ) -> PteroResult<FractalItem<DatabaseAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.create_database(server, req).await
    }
    pub async fn rotate(
        &self,
        server: &str,
        database: &str,
    ) -> PteroResult<FractalItem<DatabaseAttributes>> {
        self.0.require_client_ready()?;
        self.0
            .panel
            .rotate_database_password(server, database)
            .await
    }
    pub async fn delete(&self, server: &str, database: &str) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.delete_database(server, database).await
    }
}

impl ScheduleService<'_> {
    pub async fn list(&self, server: &str) -> PteroResult<FractalList<ScheduleAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.list_schedules(server).await
    }
    pub async fn create(
        &self,
        server: &str,
        req: &CreateScheduleRequest,
    ) -> PteroResult<FractalItem<ScheduleAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.create_schedule(server, req).await
    }
    pub async fn get(
        &self,
        server: &str,
        schedule: u64,
    ) -> PteroResult<FractalItem<ScheduleAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.get_schedule(server, schedule).await
    }
    pub async fn update(
        &self,
        server: &str,
        schedule: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<ScheduleAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.update_schedule(server, schedule, body).await
    }
    pub async fn execute(&self, server: &str, schedule: u64) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.execute_schedule(server, schedule).await
    }
    pub async fn delete(&self, server: &str, schedule: u64) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.delete_schedule(server, schedule).await
    }
    pub async fn create_task(
        &self,
        server: &str,
        schedule: u64,
        req: &CreateTaskRequest,
    ) -> PteroResult<FractalItem<TaskAttributes>> {
        self.0.require_client_ready()?;
        self.0
            .panel
            .create_schedule_task(server, schedule, req)
            .await
    }
    pub async fn update_task(
        &self,
        server: &str,
        schedule: u64,
        task: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<TaskAttributes>> {
        self.0.require_client_ready()?;
        self.0
            .panel
            .update_schedule_task(server, schedule, task, body)
            .await
    }
    pub async fn delete_task(&self, server: &str, schedule: u64, task: u64) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0
            .panel
            .delete_schedule_task(server, schedule, task)
            .await
    }
}

impl NetworkService<'_> {
    pub async fn list(&self, server: &str) -> PteroResult<FractalList<AllocationAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.list_allocations(server).await
    }
    pub async fn create(&self, server: &str) -> PteroResult<FractalItem<AllocationAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.create_allocation(server).await
    }
    pub async fn update(
        &self,
        server: &str,
        allocation: u64,
        req: &UpdateAllocationRequest,
    ) -> PteroResult<FractalItem<AllocationAttributes>> {
        self.0.require_client_ready()?;
        self.0
            .panel
            .update_allocation(server, allocation, req)
            .await
    }
    pub async fn set_primary(&self, server: &str, allocation: u64) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0
            .panel
            .set_primary_allocation(server, allocation)
            .await
    }
    pub async fn delete(&self, server: &str, allocation: u64) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.delete_allocation(server, allocation).await
    }
}

impl SubuserService<'_> {
    pub async fn list(&self, server: &str) -> PteroResult<FractalList<SubuserAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.list_subusers(server).await
    }
    pub async fn create(
        &self,
        server: &str,
        req: &SubuserRequest,
    ) -> PteroResult<FractalItem<SubuserAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.create_subuser(server, req).await
    }
    pub async fn get(
        &self,
        server: &str,
        user: &str,
    ) -> PteroResult<FractalItem<SubuserAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.get_subuser(server, user).await
    }
    pub async fn update(
        &self,
        server: &str,
        user: &str,
        req: &SubuserRequest,
    ) -> PteroResult<FractalItem<SubuserAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.update_subuser(server, user, req).await
    }
    pub async fn delete(&self, server: &str, user: &str) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.delete_subuser(server, user).await
    }
}

impl BackupService<'_> {
    pub async fn list(&self, server: &str) -> PteroResult<FractalList<BackupAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.list_backups(server).await
    }
    pub async fn create(
        &self,
        server: &str,
        req: &CreateBackupRequest,
    ) -> PteroResult<FractalItem<BackupAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.create_backup(server, req).await
    }
    pub async fn get(
        &self,
        server: &str,
        backup: &str,
    ) -> PteroResult<FractalItem<BackupAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.get_backup(server, backup).await
    }
    pub async fn download(
        &self,
        server: &str,
        backup: &str,
    ) -> PteroResult<FractalItem<ptero_protocol::files::SignedUrlAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.backup_download(server, backup).await
    }
    pub async fn toggle_lock(
        &self,
        server: &str,
        backup: &str,
    ) -> PteroResult<FractalItem<BackupAttributes>> {
        self.0.require_client_ready()?;
        self.0.panel.toggle_backup_lock(server, backup).await
    }
    pub async fn restore(
        &self,
        server: &str,
        backup: &str,
        req: &RestoreBackupRequest,
    ) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.restore_backup(server, backup, req).await
    }
    pub async fn delete(&self, server: &str, backup: &str) -> PteroResult<()> {
        self.0.require_client_ready()?;
        self.0.panel.delete_backup(server, backup).await
    }
}

impl ApplicationService<'_> {
    pub async fn list_users(
        &self,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<AppUserAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.list_users(query).await
    }
    pub async fn get_user(&self, id: u64) -> PteroResult<FractalItem<AppUserAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.get_user(id).await
    }
    pub async fn get_user_external(
        &self,
        external_id: &str,
    ) -> PteroResult<FractalItem<AppUserAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.get_user_external(external_id).await
    }
    pub async fn create_user(
        &self,
        req: &CreateAppUserRequest,
    ) -> PteroResult<FractalItem<AppUserAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.create_user(req).await
    }
    pub async fn update_user(
        &self,
        id: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<AppUserAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.update_user(id, body).await
    }
    pub async fn delete_user(&self, id: u64) -> PteroResult<()> {
        self.0.require_application_ready()?;
        self.0.panel.delete_user(id).await
    }
    pub async fn list_nodes(
        &self,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<AppNodeAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.list_nodes(query).await
    }
    pub async fn get_node(&self, id: u64) -> PteroResult<FractalItem<AppNodeAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.get_node(id).await
    }
    pub async fn deployable_nodes(&self, query: &[(String, String)]) -> PteroResult<Value> {
        self.0.require_application_ready()?;
        self.0.panel.deployable_nodes(query).await
    }
    pub async fn node_configuration(&self, id: u64) -> PteroResult<Value> {
        self.0.require_application_ready()?;
        self.0.panel.node_configuration(id).await
    }
    pub async fn create_node(&self, body: &Value) -> PteroResult<FractalItem<AppNodeAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.create_node(body).await
    }
    pub async fn update_node(
        &self,
        id: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<AppNodeAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.update_node(id, body).await
    }
    pub async fn delete_node(&self, id: u64) -> PteroResult<()> {
        self.0.require_application_ready()?;
        self.0.panel.delete_node(id).await
    }
    pub async fn list_node_allocations(
        &self,
        node: u64,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<AppAllocationAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.list_node_allocations(node, query).await
    }
    pub async fn create_node_allocations(&self, node: u64, body: &Value) -> PteroResult<Value> {
        self.0.require_application_ready()?;
        self.0.panel.create_node_allocations(node, body).await
    }
    pub async fn delete_node_allocation(&self, node: u64, allocation: u64) -> PteroResult<()> {
        self.0.require_application_ready()?;
        self.0.panel.delete_node_allocation(node, allocation).await
    }
    pub async fn list_locations(
        &self,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<AppLocationAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.list_locations(query).await
    }
    pub async fn get_location(&self, id: u64) -> PteroResult<FractalItem<AppLocationAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.get_location(id).await
    }
    pub async fn create_location(
        &self,
        req: &CreateLocationRequest,
    ) -> PteroResult<FractalItem<AppLocationAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.create_location(req).await
    }
    pub async fn update_location(
        &self,
        id: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<AppLocationAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.update_location(id, body).await
    }
    pub async fn delete_location(&self, id: u64) -> PteroResult<()> {
        self.0.require_application_ready()?;
        self.0.panel.delete_location(id).await
    }
    pub async fn list_servers(
        &self,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<AppServerAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.list_app_servers(query).await
    }
    pub async fn get_server(&self, id: u64) -> PteroResult<FractalItem<AppServerAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.get_app_server(id).await
    }
    pub async fn get_server_external(
        &self,
        external_id: &str,
    ) -> PteroResult<FractalItem<AppServerAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.get_app_server_external(external_id).await
    }
    pub async fn create_server(
        &self,
        body: &Value,
    ) -> PteroResult<FractalItem<AppServerAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.create_app_server(body).await
    }
    pub async fn update_server_details(
        &self,
        id: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<AppServerAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.update_app_server_details(id, body).await
    }
    pub async fn update_server_build(
        &self,
        id: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<AppServerAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.update_app_server_build(id, body).await
    }
    pub async fn update_server_startup(
        &self,
        id: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<AppServerAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.update_app_server_startup(id, body).await
    }
    pub async fn suspend_server(&self, id: u64) -> PteroResult<()> {
        self.0.require_application_ready()?;
        self.0.panel.suspend_app_server(id).await
    }
    pub async fn unsuspend_server(&self, id: u64) -> PteroResult<()> {
        self.0.require_application_ready()?;
        self.0.panel.unsuspend_app_server(id).await
    }
    pub async fn reinstall_server(&self, id: u64) -> PteroResult<()> {
        self.0.require_application_ready()?;
        self.0.panel.reinstall_app_server(id).await
    }
    pub async fn delete_server(&self, id: u64, force: bool) -> PteroResult<()> {
        self.0.require_application_ready()?;
        self.0.panel.delete_app_server(id, force).await
    }
    pub async fn list_databases(
        &self,
        server: u64,
    ) -> PteroResult<FractalList<AppDatabaseAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.list_app_databases(server).await
    }
    pub async fn get_database(
        &self,
        server: u64,
        database: u64,
    ) -> PteroResult<FractalItem<AppDatabaseAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.get_app_database(server, database).await
    }
    pub async fn create_database(
        &self,
        server: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<AppDatabaseAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.create_app_database(server, body).await
    }
    pub async fn reset_database_password(&self, server: u64, database: u64) -> PteroResult<()> {
        self.0.require_application_ready()?;
        self.0
            .panel
            .reset_app_database_password(server, database)
            .await
    }
    pub async fn delete_database(&self, server: u64, database: u64) -> PteroResult<()> {
        self.0.require_application_ready()?;
        self.0.panel.delete_app_database(server, database).await
    }
    pub async fn list_nests(
        &self,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<AppNestAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.list_nests(query).await
    }
    pub async fn get_nest(&self, id: u64) -> PteroResult<FractalItem<AppNestAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.get_nest(id).await
    }
    pub async fn list_eggs(
        &self,
        nest: u64,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<AppEggAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.list_eggs(nest, query).await
    }
    pub async fn get_egg(&self, nest: u64, egg: u64) -> PteroResult<FractalItem<AppEggAttributes>> {
        self.0.require_application_ready()?;
        self.0.panel.get_egg(nest, egg).await
    }
}

impl RemoteService<'_> {
    pub async fn sftp_auth(&self, req: &SftpAuthRequest) -> PteroResult<SftpAuthResponse> {
        self.0.require_remote_ready()?;
        self.0.panel.sftp_auth(req).await
    }
    pub async fn list_servers(&self, query: &[(String, String)]) -> PteroResult<Value> {
        self.0.require_remote_ready()?;
        self.0.panel.list_remote_servers(query).await
    }
    pub async fn reset_servers(&self) -> PteroResult<()> {
        self.0.require_remote_ready()?;
        self.0.panel.reset_remote_servers().await
    }
    pub async fn ingest_activity(&self, body: &Value) -> PteroResult<()> {
        self.0.require_remote_ready()?;
        self.0.panel.ingest_activity(body).await
    }
    pub async fn server_details(&self, uuid: &str) -> PteroResult<Value> {
        self.0.require_remote_ready()?;
        self.0.panel.remote_server_details(uuid).await
    }
    pub async fn install_details(&self, uuid: &str) -> PteroResult<RemoteInstallDetails> {
        self.0.require_remote_ready()?;
        self.0.panel.remote_install_details(uuid).await
    }
    pub async fn install_complete(
        &self,
        uuid: &str,
        req: &RemoteInstallCompleteRequest,
    ) -> PteroResult<()> {
        self.0.require_remote_ready()?;
        self.0.panel.remote_install_complete(uuid, req).await
    }
    pub async fn transfer_failure(&self, uuid: &str) -> PteroResult<()> {
        self.0.require_remote_ready()?;
        self.0.panel.remote_transfer_failure(uuid).await
    }
    pub async fn transfer_success(&self, uuid: &str) -> PteroResult<()> {
        self.0.require_remote_ready()?;
        self.0.panel.remote_transfer_success(uuid).await
    }
    pub async fn backup_upload(&self, backup: &str) -> PteroResult<Value> {
        self.0.require_remote_ready()?;
        self.0.panel.remote_backup_upload(backup).await
    }
    pub async fn backup_status(
        &self,
        backup: &str,
        req: &RemoteBackupStatusRequest,
    ) -> PteroResult<()> {
        self.0.require_remote_ready()?;
        self.0.panel.remote_backup_status(backup, req).await
    }
    pub async fn backup_restore(
        &self,
        backup: &str,
        req: &RemoteBackupRestoreRequest,
    ) -> PteroResult<()> {
        self.0.require_remote_ready()?;
        self.0.panel.remote_backup_restore(backup, req).await
    }
}

impl TerminalService<'_> {
    pub async fn websocket_credentials(&self, server: &str) -> PteroResult<WebsocketCredentials> {
        self.0.require_client_ready()?;
        self.0.panel.server_websocket(server).await
    }

    /// Attach to Wings console; refreshes JWT via Client websocket endpoint.
    pub async fn attach(&self, server: &str) -> PteroResult<WingsConsole> {
        self.0.require_client_ready()?;
        let creds = self.0.panel.server_websocket(server).await?;
        let panel = Arc::clone(&self.0.panel);
        let server = server.to_string();
        let refresh: TokenRefresh = Arc::new(move || {
            let panel = Arc::clone(&panel);
            let server = server.clone();
            Box::pin(async move { panel.server_websocket(&server).await })
        });
        WingsConsole::connect(creds, refresh).await
    }

    pub async fn send_command(console: &WingsConsole, command: &str) -> PteroResult<()> {
        console.send_command(command).await
    }

    pub async fn set_state(console: &WingsConsole, action: &str) -> PteroResult<()> {
        console.set_state(action).await
    }

    pub async fn next_event(console: &mut WingsConsole) -> Option<ConsoleEvent> {
        console.next().await
    }
}
