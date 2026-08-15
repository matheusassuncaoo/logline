mod domain;
mod persistence;

use domain::{Board, BoardSummary, WorkspaceSummary};
use persistence::{validate_name, Persistence};
use std::sync::Mutex;
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

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let persistence = Persistence::new(&app.handle()).map_err(std::io::Error::other)?;
            app.manage(AppState {
                persistence: Mutex::new(persistence),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_workspaces,
            create_workspace,
            create_board,
            list_boards,
            open_board,
            save_board
        ])
        .run(tauri::generate_context!())
        .expect("error while running LogLine");
}
