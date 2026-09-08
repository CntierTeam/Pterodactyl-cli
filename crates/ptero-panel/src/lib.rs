//! Pterodactyl Panel HTTP client.
//!
//! All Panel `/api` access goes through [`ClientApi`], [`ApplicationApi`], or
//! [`RemoteApi`]. CLI/TUI must not call HTTP directly.

mod application_api;
mod client;
mod client_api;
mod remote_api;
mod traits;

pub use client::{ApiSurface, HttpPanelClient};
pub use traits::{ApplicationApi, ClientApi, RemoteApi};
