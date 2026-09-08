//! ClientApi implementation for HttpPanelClient.

use crate::client::{ApiSurface, HttpPanelClient};
use crate::ClientApi;
use async_trait::async_trait;
use ptero_protocol::account::*;
use ptero_protocol::backup::*;
use ptero_protocol::files::*;
use ptero_protocol::network::*;
use ptero_protocol::schedule::*;
use ptero_protocol::server::*;
use ptero_protocol::{
    FractalItem, FractalList, PteroResult, WebsocketCredentials, WebsocketResponse,
};
use serde_json::Value;

#[async_trait]
impl ClientApi for HttpPanelClient {
    async fn raw(
        &self,
        method: &str,
        path: &str,
        query: &[(String, String)],
        body: Option<&Value>,
    ) -> PteroResult<Value> {
        self.request_raw(ApiSurface::Client, method, path, query, body)
            .await
    }

    async fn list_servers(
        &self,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<ServerAttributes>> {
        self.get_json(ApiSurface::Client, "/api/client", query)
            .await
    }

    async fn permissions(&self) -> PteroResult<Value> {
        self.get_json(ApiSurface::Client, "/api/client/permissions", &[])
            .await
    }

    async fn account(&self) -> PteroResult<FractalItem<AccountAttributes>> {
        self.get_json(ApiSurface::Client, "/api/client/account", &[])
            .await
    }

    async fn update_email(&self, req: &UpdateEmailRequest) -> PteroResult<()> {
        self.put_empty_body(ApiSurface::Client, "/api/client/account/email", req)
            .await
    }

    async fn update_password(&self, req: &UpdatePasswordRequest) -> PteroResult<()> {
        self.put_empty_body(ApiSurface::Client, "/api/client/account/password", req)
            .await
    }

    async fn two_factor_details(&self) -> PteroResult<Value> {
        self.get_json(ApiSurface::Client, "/api/client/account/two-factor", &[])
            .await
    }

    async fn enable_two_factor(&self, req: &EnableTwoFactorRequest) -> PteroResult<Value> {
        self.post_json_body(ApiSurface::Client, "/api/client/account/two-factor", req)
            .await
    }

    async fn disable_two_factor(&self, req: &DisableTwoFactorRequest) -> PteroResult<()> {
        self.post_empty_body(
            ApiSurface::Client,
            "/api/client/account/two-factor/disable",
            req,
        )
        .await
    }

    async fn account_activity(
        &self,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<ActivityAttributes>> {
        self.get_json(ApiSurface::Client, "/api/client/account/activity", query)
            .await
    }

    async fn list_api_keys(&self) -> PteroResult<FractalList<ApiKeyAttributes>> {
        self.get_json(ApiSurface::Client, "/api/client/account/api-keys", &[])
            .await
    }

    async fn create_api_key(
        &self,
        req: &CreateApiKeyRequest,
    ) -> PteroResult<FractalItem<ApiKeyAttributes>> {
        self.post_json_body(ApiSurface::Client, "/api/client/account/api-keys", req)
            .await
    }

    async fn delete_api_key(&self, identifier: &str) -> PteroResult<()> {
        self.delete_empty(
            ApiSurface::Client,
            &format!("/api/client/account/api-keys/{identifier}"),
        )
        .await
    }

    async fn list_ssh_keys(&self) -> PteroResult<FractalList<SshKeyAttributes>> {
        self.get_json(ApiSurface::Client, "/api/client/account/ssh-keys", &[])
            .await
    }

    async fn create_ssh_key(
        &self,
        req: &CreateSshKeyRequest,
    ) -> PteroResult<FractalItem<SshKeyAttributes>> {
        self.post_json_body(ApiSurface::Client, "/api/client/account/ssh-keys", req)
            .await
    }

    async fn delete_ssh_key(&self, req: &DeleteSshKeyRequest) -> PteroResult<()> {
        self.post_empty_body(
            ApiSurface::Client,
            "/api/client/account/ssh-keys/remove",
            req,
        )
        .await
    }

    async fn get_server(&self, server: &str) -> PteroResult<FractalItem<ServerAttributes>> {
        self.get_json(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}"),
            &[],
        )
        .await
    }

    async fn server_websocket(&self, server: &str) -> PteroResult<WebsocketCredentials> {
        let resp: WebsocketResponse = self
            .get_json(
                ApiSurface::Client,
                &format!("/api/client/servers/{server}/websocket"),
                &[],
            )
            .await?;
        Ok(resp.into_credentials())
    }

    async fn server_resources(&self, server: &str) -> PteroResult<FractalItem<ResourceStats>> {
        self.get_json(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/resources"),
            &[],
        )
        .await
    }

    async fn server_activity(
        &self,
        server: &str,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<ActivityAttributes>> {
        self.get_json(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/activity"),
            query,
        )
        .await
    }

    async fn server_command(&self, server: &str, command: &str) -> PteroResult<()> {
        let req = CommandRequest {
            command: command.to_string(),
        };
        self.post_empty_body(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/command"),
            &req,
        )
        .await
    }

    async fn server_power(&self, server: &str, signal: PowerSignal) -> PteroResult<()> {
        let req = PowerRequest {
            signal: signal.as_str().to_string(),
        };
        self.post_empty_body(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/power"),
            &req,
        )
        .await
    }

    async fn list_databases(&self, server: &str) -> PteroResult<FractalList<DatabaseAttributes>> {
        self.get_json(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/databases"),
            &[],
        )
        .await
    }

    async fn create_database(
        &self,
        server: &str,
        req: &CreateDatabaseRequest,
    ) -> PteroResult<FractalItem<DatabaseAttributes>> {
        self.post_json_body(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/databases"),
            req,
        )
        .await
    }

    async fn rotate_database_password(
        &self,
        server: &str,
        database: &str,
    ) -> PteroResult<FractalItem<DatabaseAttributes>> {
        self.request_json(
            ApiSurface::Client,
            "POST",
            &format!("/api/client/servers/{server}/databases/{database}/rotate-password"),
            &[],
            None,
        )
        .await
    }

    async fn delete_database(&self, server: &str, database: &str) -> PteroResult<()> {
        self.delete_empty(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/databases/{database}"),
        )
        .await
    }

    async fn list_files(
        &self,
        server: &str,
        directory: &str,
    ) -> PteroResult<FractalList<FileEntry>> {
        let q = vec![("directory".into(), directory.to_string())];
        self.get_json(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/files/list"),
            &q,
        )
        .await
    }

    async fn file_contents(&self, server: &str, file: &str) -> PteroResult<String> {
        let q = vec![("file".into(), file.to_string())];
        self.get_plain(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/files/contents"),
            &q,
        )
        .await
    }

    async fn file_download(
        &self,
        server: &str,
        file: &str,
    ) -> PteroResult<FractalItem<SignedUrlAttributes>> {
        let q = vec![("file".into(), file.to_string())];
        self.get_json(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/files/download"),
            &q,
        )
        .await
    }

    async fn file_rename(&self, server: &str, req: &RenameFileRequest) -> PteroResult<()> {
        self.put_empty_body(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/files/rename"),
            req,
        )
        .await
    }

    async fn file_copy(&self, server: &str, req: &CopyFileRequest) -> PteroResult<()> {
        self.post_empty_body(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/files/copy"),
            req,
        )
        .await
    }

    async fn file_write(&self, server: &str, file: &str, contents: &str) -> PteroResult<()> {
        let q = vec![("file".into(), file.to_string())];
        self.post_raw_body(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/files/write"),
            &q,
            contents,
        )
        .await
    }

    async fn file_compress(
        &self,
        server: &str,
        req: &CompressFilesRequest,
    ) -> PteroResult<FractalItem<FileEntry>> {
        self.post_json_body(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/files/compress"),
            req,
        )
        .await
    }

    async fn file_decompress(&self, server: &str, req: &DecompressFileRequest) -> PteroResult<()> {
        self.post_empty_body(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/files/decompress"),
            req,
        )
        .await
    }

    async fn file_delete(&self, server: &str, req: &DeleteFilesRequest) -> PteroResult<()> {
        self.post_empty_body(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/files/delete"),
            req,
        )
        .await
    }

    async fn file_create_folder(&self, server: &str, req: &CreateFolderRequest) -> PteroResult<()> {
        self.post_empty_body(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/files/create-folder"),
            req,
        )
        .await
    }

    async fn file_chmod(&self, server: &str, req: &ChmodFilesRequest) -> PteroResult<()> {
        self.post_empty_body(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/files/chmod"),
            req,
        )
        .await
    }

    async fn file_pull(&self, server: &str, req: &PullFileRequest) -> PteroResult<()> {
        self.post_empty_body(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/files/pull"),
            req,
        )
        .await
    }

    async fn file_upload_url(&self, server: &str) -> PteroResult<FractalItem<SignedUrlAttributes>> {
        self.get_json(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/files/upload"),
            &[],
        )
        .await
    }

    async fn list_schedules(&self, server: &str) -> PteroResult<FractalList<ScheduleAttributes>> {
        self.get_json(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/schedules"),
            &[],
        )
        .await
    }

    async fn create_schedule(
        &self,
        server: &str,
        req: &CreateScheduleRequest,
    ) -> PteroResult<FractalItem<ScheduleAttributes>> {
        self.post_json_body(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/schedules"),
            req,
        )
        .await
    }

    async fn get_schedule(
        &self,
        server: &str,
        schedule: u64,
    ) -> PteroResult<FractalItem<ScheduleAttributes>> {
        self.get_json(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/schedules/{schedule}"),
            &[],
        )
        .await
    }

    async fn update_schedule(
        &self,
        server: &str,
        schedule: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<ScheduleAttributes>> {
        self.request_json(
            ApiSurface::Client,
            "POST",
            &format!("/api/client/servers/{server}/schedules/{schedule}"),
            &[],
            Some(body),
        )
        .await
    }

    async fn execute_schedule(&self, server: &str, schedule: u64) -> PteroResult<()> {
        self.request_empty(
            ApiSurface::Client,
            "POST",
            &format!("/api/client/servers/{server}/schedules/{schedule}/execute"),
            &[],
            None,
        )
        .await
    }

    async fn delete_schedule(&self, server: &str, schedule: u64) -> PteroResult<()> {
        self.delete_empty(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/schedules/{schedule}"),
        )
        .await
    }

    async fn create_schedule_task(
        &self,
        server: &str,
        schedule: u64,
        req: &CreateTaskRequest,
    ) -> PteroResult<FractalItem<TaskAttributes>> {
        self.post_json_body(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/schedules/{schedule}/tasks"),
            req,
        )
        .await
    }

    async fn update_schedule_task(
        &self,
        server: &str,
        schedule: u64,
        task: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<TaskAttributes>> {
        self.request_json(
            ApiSurface::Client,
            "POST",
            &format!("/api/client/servers/{server}/schedules/{schedule}/tasks/{task}"),
            &[],
            Some(body),
        )
        .await
    }

    async fn delete_schedule_task(
        &self,
        server: &str,
        schedule: u64,
        task: u64,
    ) -> PteroResult<()> {
        self.delete_empty(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/schedules/{schedule}/tasks/{task}"),
        )
        .await
    }

    async fn list_allocations(
        &self,
        server: &str,
    ) -> PteroResult<FractalList<AllocationAttributes>> {
        self.get_json(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/network/allocations"),
            &[],
        )
        .await
    }

    async fn create_allocation(
        &self,
        server: &str,
    ) -> PteroResult<FractalItem<AllocationAttributes>> {
        self.request_json(
            ApiSurface::Client,
            "POST",
            &format!("/api/client/servers/{server}/network/allocations"),
            &[],
            None,
        )
        .await
    }

    async fn update_allocation(
        &self,
        server: &str,
        allocation: u64,
        req: &UpdateAllocationRequest,
    ) -> PteroResult<FractalItem<AllocationAttributes>> {
        self.post_json_body(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/network/allocations/{allocation}"),
            req,
        )
        .await
    }

    async fn set_primary_allocation(&self, server: &str, allocation: u64) -> PteroResult<()> {
        self.request_empty(
            ApiSurface::Client,
            "POST",
            &format!("/api/client/servers/{server}/network/allocations/{allocation}/primary"),
            &[],
            None,
        )
        .await
    }

    async fn delete_allocation(&self, server: &str, allocation: u64) -> PteroResult<()> {
        self.delete_empty(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/network/allocations/{allocation}"),
        )
        .await
    }

    async fn list_subusers(&self, server: &str) -> PteroResult<FractalList<SubuserAttributes>> {
        self.get_json(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/users"),
            &[],
        )
        .await
    }

    async fn create_subuser(
        &self,
        server: &str,
        req: &SubuserRequest,
    ) -> PteroResult<FractalItem<SubuserAttributes>> {
        self.post_json_body(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/users"),
            req,
        )
        .await
    }

    async fn get_subuser(
        &self,
        server: &str,
        user: &str,
    ) -> PteroResult<FractalItem<SubuserAttributes>> {
        self.get_json(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/users/{user}"),
            &[],
        )
        .await
    }

    async fn update_subuser(
        &self,
        server: &str,
        user: &str,
        req: &SubuserRequest,
    ) -> PteroResult<FractalItem<SubuserAttributes>> {
        self.post_json_body(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/users/{user}"),
            req,
        )
        .await
    }

    async fn delete_subuser(&self, server: &str, user: &str) -> PteroResult<()> {
        self.delete_empty(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/users/{user}"),
        )
        .await
    }

    async fn list_backups(&self, server: &str) -> PteroResult<FractalList<BackupAttributes>> {
        self.get_json(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/backups"),
            &[],
        )
        .await
    }

    async fn create_backup(
        &self,
        server: &str,
        req: &CreateBackupRequest,
    ) -> PteroResult<FractalItem<BackupAttributes>> {
        self.post_json_body(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/backups"),
            req,
        )
        .await
    }

    async fn get_backup(
        &self,
        server: &str,
        backup: &str,
    ) -> PteroResult<FractalItem<BackupAttributes>> {
        self.get_json(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/backups/{backup}"),
            &[],
        )
        .await
    }

    async fn backup_download(
        &self,
        server: &str,
        backup: &str,
    ) -> PteroResult<FractalItem<SignedUrlAttributes>> {
        self.get_json(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/backups/{backup}/download"),
            &[],
        )
        .await
    }

    async fn toggle_backup_lock(
        &self,
        server: &str,
        backup: &str,
    ) -> PteroResult<FractalItem<BackupAttributes>> {
        self.request_json(
            ApiSurface::Client,
            "POST",
            &format!("/api/client/servers/{server}/backups/{backup}/lock"),
            &[],
            None,
        )
        .await
    }

    async fn restore_backup(
        &self,
        server: &str,
        backup: &str,
        req: &RestoreBackupRequest,
    ) -> PteroResult<()> {
        self.post_empty_body(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/backups/{backup}/restore"),
            req,
        )
        .await
    }

    async fn delete_backup(&self, server: &str, backup: &str) -> PteroResult<()> {
        self.delete_empty(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/backups/{backup}"),
        )
        .await
    }

    async fn startup(&self, server: &str) -> PteroResult<Value> {
        self.get_json(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/startup"),
            &[],
        )
        .await
    }

    async fn update_startup_variable(
        &self,
        server: &str,
        req: &UpdateStartupVariableRequest,
    ) -> PteroResult<Value> {
        self.put_json_body(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/startup/variable"),
            req,
        )
        .await
    }

    async fn update_startup_egg(&self, server: &str, body: &Value) -> PteroResult<Value> {
        self.request_json(
            ApiSurface::Client,
            "PUT",
            &format!("/api/client/servers/{server}/startup/egg"),
            &[],
            Some(body),
        )
        .await
    }

    async fn rename_server(&self, server: &str, req: &RenameServerRequest) -> PteroResult<()> {
        self.post_empty_body(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/settings/rename"),
            req,
        )
        .await
    }

    async fn reinstall_server(&self, server: &str) -> PteroResult<()> {
        self.request_empty(
            ApiSurface::Client,
            "POST",
            &format!("/api/client/servers/{server}/settings/reinstall"),
            &[],
            None,
        )
        .await
    }

    async fn set_docker_image(&self, server: &str, req: &DockerImageRequest) -> PteroResult<()> {
        self.put_empty_body(
            ApiSurface::Client,
            &format!("/api/client/servers/{server}/settings/docker-image"),
            req,
        )
        .await
    }
}
