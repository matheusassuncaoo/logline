mod domain;
mod persistence;

use domain::{AppPreferences, AssetData, Board, BoardSummary, WorkspaceSummary};
use persistence::{validate_name, Persistence};
use std::{fs, sync::Mutex};
use tauri::{Manager, State};

struct AppState {
    persistence: Mutex<Persistence>,
}

#[tauri::command]
fn list_workspaces(state: State<'_, AppState>) -> Result<Vec<WorkspaceSummary>, String> {
    state
        .persistence
        .lock()
        .map_err(|_| "O armazenamento está indisponível.".to_owned())?
        .list_workspaces()
}

#[tauri::command]
fn create_workspace(name: String, state: State<'_, AppState>) -> Result<WorkspaceSummary, String> {
    let name = name.trim().to_owned();
    validate_name(&name)?;
    state
        .persistence
        .lock()
        .map_err(|_| "O armazenamento está indisponível.".to_owned())?
        .create_workspace(name)
}

#[tauri::command]
fn get_preferences(state: State<'_, AppState>) -> Result<AppPreferences, String> {
    state
        .persistence
        .lock()
        .map_err(|_| "O armazenamento está indisponível.".to_owned())?
        .get_preferences()
}

#[tauri::command]
fn save_preferences(
    preferences: AppPreferences,
    state: State<'_, AppState>,
) -> Result<AppPreferences, String> {
    state
        .persistence
        .lock()
        .map_err(|_| "O armazenamento está indisponível.".to_owned())?
        .save_preferences(preferences)
}

#[tauri::command]
fn create_board(
    workspace_id: String,
    name: String,
    state: State<'_, AppState>,
) -> Result<Board, String> {
    let name = name.trim().to_owned();
    validate_name(&name)?;
    state
        .persistence
        .lock()
        .map_err(|_| "O armazenamento está indisponível.".to_owned())?
        .create_board(&workspace_id, name)
}

#[tauri::command]
fn rename_board(
    workspace_id: String,
    board_id: String,
    name: String,
    state: State<'_, AppState>,
) -> Result<Board, String> {
    let name = name.trim().to_owned();
    validate_name(&name)?;
    state
        .persistence
        .lock()
        .map_err(|_| "O armazenamento está indisponível.".to_owned())?
        .rename_board(&workspace_id, &board_id, name)
}

#[tauri::command]
fn duplicate_board(
    workspace_id: String,
    board_id: String,
    name: String,
    state: State<'_, AppState>,
) -> Result<Board, String> {
    let name = name.trim().to_owned();
    validate_name(&name)?;
    state
        .persistence
        .lock()
        .map_err(|_| "O armazenamento está indisponível.".to_owned())?
        .duplicate_board(&workspace_id, &board_id, name)
}

#[tauri::command]
fn delete_board(
    workspace_id: String,
    board_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state
        .persistence
        .lock()
        .map_err(|_| "O armazenamento está indisponível.".to_owned())?
        .delete_board(&workspace_id, &board_id)
}

#[tauri::command]
fn list_boards(
    workspace_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<BoardSummary>, String> {
    state
        .persistence
        .lock()
        .map_err(|_| "O armazenamento está indisponível.".to_owned())?
        .list_boards(&workspace_id)
}

#[tauri::command]
fn open_board(
    workspace_id: String,
    board_id: String,
    state: State<'_, AppState>,
) -> Result<Board, String> {
    state
        .persistence
        .lock()
        .map_err(|_| "O armazenamento está indisponível.".to_owned())?
        .open_board(&workspace_id, &board_id)
}

#[tauri::command]
fn save_board(board: Board, state: State<'_, AppState>) -> Result<Board, String> {
    state
        .persistence
        .lock()
        .map_err(|_| "O armazenamento está indisponível.".to_owned())?
        .save_board(board)
}

#[tauri::command]
fn add_asset(
    workspace_id: String,
    file_name: String,
    mime_type: String,
    bytes: Vec<u8>,
    state: State<'_, AppState>,
) -> Result<AssetData, String> {
    state
        .persistence
        .lock()
        .map_err(|_| "O armazenamento está indisponível.".to_owned())?
        .add_asset(&workspace_id, &file_name, &mime_type, &bytes)
}

#[tauri::command]
fn read_asset(
    workspace_id: String,
    asset_id: String,
    state: State<'_, AppState>,
) -> Result<AssetData, String> {
    state
        .persistence
        .lock()
        .map_err(|_| "O armazenamento está indisponível.".to_owned())?
        .read_asset(&workspace_id, &asset_id)
}

#[tauri::command]
fn export_workspace(workspace_id: String, state: State<'_, AppState>) -> Result<Vec<u8>, String> {
    state
        .persistence
        .lock()
        .map_err(|_| "O armazenamento está indisponível.".to_owned())?
        .export_workspace(&workspace_id)
}

#[tauri::command]
fn import_workspace(
    bytes: Vec<u8>,
    state: State<'_, AppState>,
) -> Result<WorkspaceSummary, String> {
    state
        .persistence
        .lock()
        .map_err(|_| "O armazenamento está indisponível.".to_owned())?
        .import_workspace(&bytes)
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let log_directory = app.path().app_log_dir()?;
            fs::create_dir_all(&log_directory)?;
            let log_writer = tracing_appender::rolling::daily(log_directory, "logline");
            let _ = tracing_subscriber::fmt()
                .with_ansi(false)
                .with_writer(log_writer)
                .try_init();
            tracing::info!("LogLine initialized");
            let persistence = Persistence::new(app.handle()).map_err(std::io::Error::other)?;
            app.manage(AppState {
                persistence: Mutex::new(persistence),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_workspaces,
            create_workspace,
            get_preferences,
            save_preferences,
            create_board,
            rename_board,
            duplicate_board,
            delete_board,
            list_boards,
            open_board,
            save_board,
            add_asset,
            read_asset,
            export_workspace,
            import_workspace
        ])
        .run(tauri::generate_context!())
        .expect("error while running LogLine");
}
