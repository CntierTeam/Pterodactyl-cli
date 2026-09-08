//! RemoteApi implementation for HttpPanelClient.

use crate::client::{ApiSurface, HttpPanelClient};
use crate::RemoteApi;
use async_trait::async_trait;
use ptero_protocol::remote::*;
use ptero_protocol::PteroResult;
use serde_json::Value;

#[async_trait]
impl RemoteApi for HttpPanelClient {
    async fn raw(
        &self,
        method: &str,
        path: &str,
        query: &[(String, String)],
        body: Option<&Value>,
    ) -> PteroResult<Value> {
        self.request_raw(ApiSurface::Remote, method, path, query, body)
            .await
    }

    async fn sftp_auth(&self, req: &SftpAuthRequest) -> PteroResult<SftpAuthResponse> {
        self.post_json_body(ApiSurface::Remote, "/api/remote/sftp/auth", req)
            .await
    }

    async fn list_remote_servers(&self, query: &[(String, String)]) -> PteroResult<Value> {
        self.get_json(ApiSurface::Remote, "/api/remote/servers", query)
            .await
    }

    async fn reset_remote_servers(&self) -> PteroResult<()> {
        self.request_empty(
            ApiSurface::Remote,
            "POST",
            "/api/remote/servers/reset",
            &[],
            None,
        )
        .await
    }

    async fn ingest_activity(&self, body: &Value) -> PteroResult<()> {
        self.request_empty(
            ApiSurface::Remote,
            "POST",
            "/api/remote/activity",
            &[],
            Some(body),
        )
        .await
    }

    async fn remote_server_details(&self, uuid: &str) -> PteroResult<Value> {
        self.get_json(
            ApiSurface::Remote,
            &format!("/api/remote/servers/{uuid}"),
            &[],
        )
        .await
    }

    async fn remote_install_details(&self, uuid: &str) -> PteroResult<RemoteInstallDetails> {
        self.get_json(
            ApiSurface::Remote,
            &format!("/api/remote/servers/{uuid}/install"),
            &[],
        )
        .await
    }

    async fn remote_install_complete(
        &self,
        uuid: &str,
        req: &RemoteInstallCompleteRequest,
    ) -> PteroResult<()> {
        self.post_empty_body(
            ApiSurface::Remote,
            &format!("/api/remote/servers/{uuid}/install"),
            req,
        )
        .await
    }

    async fn remote_transfer_failure(&self, uuid: &str) -> PteroResult<()> {
        self.request_empty(
            ApiSurface::Remote,
            "POST",
            &format!("/api/remote/servers/{uuid}/transfer/failure"),
            &[],
            None,
        )
        .await
    }

    async fn remote_transfer_success(&self, uuid: &str) -> PteroResult<()> {
        self.request_empty(
            ApiSurface::Remote,
            "POST",
            &format!("/api/remote/servers/{uuid}/transfer/success"),
            &[],
            None,
        )
        .await
    }

    async fn remote_backup_upload(&self, backup: &str) -> PteroResult<Value> {
        self.get_json(
            ApiSurface::Remote,
            &format!("/api/remote/backups/{backup}"),
            &[],
        )
        .await
    }

    async fn remote_backup_status(
        &self,
        backup: &str,
        req: &RemoteBackupStatusRequest,
    ) -> PteroResult<()> {
        self.post_empty_body(
            ApiSurface::Remote,
            &format!("/api/remote/backups/{backup}"),
            req,
        )
        .await
    }

    async fn remote_backup_restore(
        &self,
        backup: &str,
        req: &RemoteBackupRestoreRequest,
    ) -> PteroResult<()> {
        self.post_empty_body(
            ApiSurface::Remote,
            &format!("/api/remote/backups/{backup}/restore"),
            req,
        )
        .await
    }
}
