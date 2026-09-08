//! ApplicationApi implementation for HttpPanelClient.

use crate::client::{ApiSurface, HttpPanelClient};
use crate::ApplicationApi;
use async_trait::async_trait;
use ptero_protocol::application::*;
use ptero_protocol::{FractalItem, FractalList, PteroResult};
use serde_json::Value;

#[async_trait]
impl ApplicationApi for HttpPanelClient {
    async fn raw(
        &self,
        method: &str,
        path: &str,
        query: &[(String, String)],
        body: Option<&Value>,
    ) -> PteroResult<Value> {
        self.request_raw(ApiSurface::Application, method, path, query, body)
            .await
    }

    async fn list_users(
        &self,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<AppUserAttributes>> {
        self.get_json(ApiSurface::Application, "/api/application/users", query)
            .await
    }

    async fn get_user(&self, id: u64) -> PteroResult<FractalItem<AppUserAttributes>> {
        self.get_json(
            ApiSurface::Application,
            &format!("/api/application/users/{id}"),
            &[],
        )
        .await
    }

    async fn get_user_external(
        &self,
        external_id: &str,
    ) -> PteroResult<FractalItem<AppUserAttributes>> {
        self.get_json(
            ApiSurface::Application,
            &format!("/api/application/users/external/{external_id}"),
            &[],
        )
        .await
    }

    async fn create_user(
        &self,
        req: &CreateAppUserRequest,
    ) -> PteroResult<FractalItem<AppUserAttributes>> {
        self.post_json_body(ApiSurface::Application, "/api/application/users", req)
            .await
    }

    async fn update_user(
        &self,
        id: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<AppUserAttributes>> {
        self.patch_json_body(
            ApiSurface::Application,
            &format!("/api/application/users/{id}"),
            body,
        )
        .await
    }

    async fn delete_user(&self, id: u64) -> PteroResult<()> {
        self.delete_empty(
            ApiSurface::Application,
            &format!("/api/application/users/{id}"),
        )
        .await
    }

    async fn list_nodes(
        &self,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<AppNodeAttributes>> {
        self.get_json(ApiSurface::Application, "/api/application/nodes", query)
            .await
    }

    async fn deployable_nodes(&self, query: &[(String, String)]) -> PteroResult<Value> {
        self.get_json(
            ApiSurface::Application,
            "/api/application/nodes/deployable",
            query,
        )
        .await
    }

    async fn get_node(&self, id: u64) -> PteroResult<FractalItem<AppNodeAttributes>> {
        self.get_json(
            ApiSurface::Application,
            &format!("/api/application/nodes/{id}"),
            &[],
        )
        .await
    }

    async fn node_configuration(&self, id: u64) -> PteroResult<Value> {
        self.get_json(
            ApiSurface::Application,
            &format!("/api/application/nodes/{id}/configuration"),
            &[],
        )
        .await
    }

    async fn create_node(&self, body: &Value) -> PteroResult<FractalItem<AppNodeAttributes>> {
        self.post_json_body(ApiSurface::Application, "/api/application/nodes", body)
            .await
    }

    async fn update_node(
        &self,
        id: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<AppNodeAttributes>> {
        self.patch_json_body(
            ApiSurface::Application,
            &format!("/api/application/nodes/{id}"),
            body,
        )
        .await
    }

    async fn delete_node(&self, id: u64) -> PteroResult<()> {
        self.delete_empty(
            ApiSurface::Application,
            &format!("/api/application/nodes/{id}"),
        )
        .await
    }

    async fn list_node_allocations(
        &self,
        node: u64,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<AppAllocationAttributes>> {
        self.get_json(
            ApiSurface::Application,
            &format!("/api/application/nodes/{node}/allocations"),
            query,
        )
        .await
    }

    async fn create_node_allocations(&self, node: u64, body: &Value) -> PteroResult<Value> {
        self.post_json_body(
            ApiSurface::Application,
            &format!("/api/application/nodes/{node}/allocations"),
            body,
        )
        .await
    }

    async fn delete_node_allocation(&self, node: u64, allocation: u64) -> PteroResult<()> {
        self.delete_empty(
            ApiSurface::Application,
            &format!("/api/application/nodes/{node}/allocations/{allocation}"),
        )
        .await
    }

    async fn list_locations(
        &self,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<AppLocationAttributes>> {
        self.get_json(ApiSurface::Application, "/api/application/locations", query)
            .await
    }

    async fn get_location(&self, id: u64) -> PteroResult<FractalItem<AppLocationAttributes>> {
        self.get_json(
            ApiSurface::Application,
            &format!("/api/application/locations/{id}"),
            &[],
        )
        .await
    }

    async fn create_location(
        &self,
        req: &CreateLocationRequest,
    ) -> PteroResult<FractalItem<AppLocationAttributes>> {
        self.post_json_body(ApiSurface::Application, "/api/application/locations", req)
            .await
    }

    async fn update_location(
        &self,
        id: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<AppLocationAttributes>> {
        self.patch_json_body(
            ApiSurface::Application,
            &format!("/api/application/locations/{id}"),
            body,
        )
        .await
    }

    async fn delete_location(&self, id: u64) -> PteroResult<()> {
        self.delete_empty(
            ApiSurface::Application,
            &format!("/api/application/locations/{id}"),
        )
        .await
    }

    async fn list_app_servers(
        &self,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<AppServerAttributes>> {
        self.get_json(ApiSurface::Application, "/api/application/servers", query)
            .await
    }

    async fn get_app_server(&self, id: u64) -> PteroResult<FractalItem<AppServerAttributes>> {
        self.get_json(
            ApiSurface::Application,
            &format!("/api/application/servers/{id}"),
            &[],
        )
        .await
    }

    async fn get_app_server_external(
        &self,
        external_id: &str,
    ) -> PteroResult<FractalItem<AppServerAttributes>> {
        self.get_json(
            ApiSurface::Application,
            &format!("/api/application/servers/external/{external_id}"),
            &[],
        )
        .await
    }

    async fn create_app_server(
        &self,
        body: &Value,
    ) -> PteroResult<FractalItem<AppServerAttributes>> {
        self.post_json_body(ApiSurface::Application, "/api/application/servers", body)
            .await
    }

    async fn update_app_server_details(
        &self,
        id: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<AppServerAttributes>> {
        self.patch_json_body(
            ApiSurface::Application,
            &format!("/api/application/servers/{id}/details"),
            body,
        )
        .await
    }

    async fn update_app_server_build(
        &self,
        id: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<AppServerAttributes>> {
        self.patch_json_body(
            ApiSurface::Application,
            &format!("/api/application/servers/{id}/build"),
            body,
        )
        .await
    }

    async fn update_app_server_startup(
        &self,
        id: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<AppServerAttributes>> {
        self.patch_json_body(
            ApiSurface::Application,
            &format!("/api/application/servers/{id}/startup"),
            body,
        )
        .await
    }

    async fn suspend_app_server(&self, id: u64) -> PteroResult<()> {
        self.request_empty(
            ApiSurface::Application,
            "POST",
            &format!("/api/application/servers/{id}/suspend"),
            &[],
            None,
        )
        .await
    }

    async fn unsuspend_app_server(&self, id: u64) -> PteroResult<()> {
        self.request_empty(
            ApiSurface::Application,
            "POST",
            &format!("/api/application/servers/{id}/unsuspend"),
            &[],
            None,
        )
        .await
    }

    async fn reinstall_app_server(&self, id: u64) -> PteroResult<()> {
        self.request_empty(
            ApiSurface::Application,
            "POST",
            &format!("/api/application/servers/{id}/reinstall"),
            &[],
            None,
        )
        .await
    }

    async fn delete_app_server(&self, id: u64, force: bool) -> PteroResult<()> {
        let path = if force {
            format!("/api/application/servers/{id}/force")
        } else {
            format!("/api/application/servers/{id}")
        };
        self.delete_empty(ApiSurface::Application, &path).await
    }

    async fn list_app_databases(
        &self,
        server: u64,
    ) -> PteroResult<FractalList<AppDatabaseAttributes>> {
        self.get_json(
            ApiSurface::Application,
            &format!("/api/application/servers/{server}/databases"),
            &[],
        )
        .await
    }

    async fn get_app_database(
        &self,
        server: u64,
        database: u64,
    ) -> PteroResult<FractalItem<AppDatabaseAttributes>> {
        self.get_json(
            ApiSurface::Application,
            &format!("/api/application/servers/{server}/databases/{database}"),
            &[],
        )
        .await
    }

    async fn create_app_database(
        &self,
        server: u64,
        body: &Value,
    ) -> PteroResult<FractalItem<AppDatabaseAttributes>> {
        self.post_json_body(
            ApiSurface::Application,
            &format!("/api/application/servers/{server}/databases"),
            body,
        )
        .await
    }

    async fn reset_app_database_password(&self, server: u64, database: u64) -> PteroResult<()> {
        self.request_empty(
            ApiSurface::Application,
            "POST",
            &format!("/api/application/servers/{server}/databases/{database}/reset-password"),
            &[],
            None,
        )
        .await
    }

    async fn delete_app_database(&self, server: u64, database: u64) -> PteroResult<()> {
        self.delete_empty(
            ApiSurface::Application,
            &format!("/api/application/servers/{server}/databases/{database}"),
        )
        .await
    }

    async fn list_nests(
        &self,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<AppNestAttributes>> {
        self.get_json(ApiSurface::Application, "/api/application/nests", query)
            .await
    }

    async fn get_nest(&self, id: u64) -> PteroResult<FractalItem<AppNestAttributes>> {
        self.get_json(
            ApiSurface::Application,
            &format!("/api/application/nests/{id}"),
            &[],
        )
        .await
    }

    async fn list_eggs(
        &self,
        nest: u64,
        query: &[(String, String)],
    ) -> PteroResult<FractalList<AppEggAttributes>> {
        self.get_json(
            ApiSurface::Application,
            &format!("/api/application/nests/{nest}/eggs"),
            query,
        )
        .await
    }

    async fn get_egg(&self, nest: u64, egg: u64) -> PteroResult<FractalItem<AppEggAttributes>> {
        self.get_json(
            ApiSurface::Application,
            &format!("/api/application/nests/{nest}/eggs/{egg}"),
            &[],
        )
        .await
    }
}
