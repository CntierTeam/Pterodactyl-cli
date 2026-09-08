//! Client file manager DTOs.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileEntry {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    pub mode_bits: String,
    #[serde(default)]
    pub size: u64,
    #[serde(default)]
    pub is_file: bool,
    #[serde(default)]
    pub is_symlink: bool,
    #[serde(default)]
    pub mimetype: String,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub modified_at: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignedUrlAttributes {
    #[serde(default)]
    pub url: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameFileRequest {
    pub root: String,
    pub files: Vec<RenameFilePair>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameFilePair {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyFileRequest {
    pub location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressFilesRequest {
    pub root: String,
    pub files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecompressFileRequest {
    pub root: String,
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFilesRequest {
    pub root: String,
    pub files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFolderRequest {
    pub root: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChmodFilesRequest {
    pub root: String,
    pub files: Vec<ChmodFileEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChmodFileEntry {
    pub file: String,
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullFileRequest {
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_header: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub foreground: Option<bool>,
}
