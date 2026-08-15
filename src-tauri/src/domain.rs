use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const WORKSPACE_SCHEMA_VERSION: u32 = 1;
pub const BOARD_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceSummary {
    pub id: String,
    pub name: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub board_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceIndex {
    pub schema_version: u32,
    pub workspaces: Vec<WorkspaceSummary>,
}

impl Default for WorkspaceIndex {
    fn default() -> Self {
        Self {
            schema_version: WORKSPACE_SCHEMA_VERSION,
            workspaces: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanvasElement {
    pub id: String,
    pub kind: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub rotation: f64,
    #[serde(default)]
    pub content: String,
    #[serde(default = "default_note_color")]
    pub color: String,
}

fn default_note_color() -> String {
    "#f7dd72".to_owned()
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BoardSummary {
    pub id: String,
    pub name: String,
    pub updated_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    pub id: String,
    pub workspace_id: String,
    pub name: String,
    pub schema_version: u32,
    pub created_at: u64,
    pub updated_at: u64,
    pub elements: BTreeMap<String, CanvasElement>,
    pub element_order: Vec<String>,
}
