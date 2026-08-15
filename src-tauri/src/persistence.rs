use crate::domain::{Board, BoardSummary, WorkspaceIndex, WorkspaceSummary, BOARD_SCHEMA_VERSION};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::{AppHandle, Manager};

#[derive(Debug)]
pub struct Persistence {
    root: PathBuf,
}

impl Persistence {
    pub fn new(app: &AppHandle) -> Result<Self, String> {
        let root = app
            .path()
            .app_local_data_dir()
            .map_err(|error| format!("Não foi possível localizar o diretório local: {error}"))?
            .join("workspaces");
        fs::create_dir_all(&root)
            .map_err(|error| format!("Não foi possível preparar o armazenamento local: {error}"))?;
        Ok(Self { root })
    }

    pub fn list_workspaces(&self) -> Result<Vec<WorkspaceSummary>, String> {
        Ok(self.read_index()?.workspaces)
    }

    pub fn create_workspace(&self, name: String) -> Result<WorkspaceSummary, String> {
        let now = now();
        let workspace = WorkspaceSummary {
            id: nanoid::nanoid!(),
            name,
            created_at: now,
            updated_at: now,
            board_count: 0,
        };
        fs::create_dir_all(self.board_directory(&workspace.id))
            .map_err(|error| format!("Não foi possível criar o workspace: {error}"))?;

        let mut index = self.read_index()?;
        index.workspaces.insert(0, workspace.clone());
        self.write_index(&index)?;
        Ok(workspace)
    }

    pub fn create_board(&self, workspace_id: &str, name: String) -> Result<Board, String> {
        self.workspace(workspace_id)?;
        let now = now();
        let board = Board {
            id: nanoid::nanoid!(),
            workspace_id: workspace_id.to_owned(),
            name,
            schema_version: BOARD_SCHEMA_VERSION,
            created_at: now,
            updated_at: now,
            elements: Default::default(),
            element_order: Vec::new(),
        };
        self.write_board(&board)?;
        self.update_workspace(workspace_id, |workspace| {
            workspace.board_count += 1;
            workspace.updated_at = now;
        })?;
        Ok(board)
    }

    pub fn list_boards(&self, workspace_id: &str) -> Result<Vec<BoardSummary>, String> {
        self.workspace(workspace_id)?;
        let directory = self.board_directory(workspace_id);
        let entries = fs::read_dir(directory)
            .map_err(|error| format!("Não foi possível listar os boards: {error}"))?;
        let mut boards = Vec::new();
        for entry in entries {
            let entry =
                entry.map_err(|error| format!("Não foi possível ler os boards: {error}"))?;
            if entry
                .path()
                .extension()
                .and_then(|extension| extension.to_str())
                != Some("json")
            {
                continue;
            }
            let file = fs::File::open(entry.path())
                .map_err(|error| format!("Não foi possível abrir um board: {error}"))?;
            let board: Board = serde_json::from_reader(file)
                .map_err(|error| format!("Um board está corrompido: {error}"))?;
            boards.push(BoardSummary {
                id: board.id,
                name: board.name,
                updated_at: board.updated_at,
            });
        }
        boards.sort_by(|left, right| right.updated_at.cmp(&left.updated_at));
        Ok(boards)
    }

    pub fn open_board(&self, workspace_id: &str, board_id: &str) -> Result<Board, String> {
        self.workspace(workspace_id)?;
        let path = self.board_path(workspace_id, board_id)?;
        let file = fs::File::open(path)
            .map_err(|error| format!("Não foi possível abrir o board: {error}"))?;
        let board: Board = serde_json::from_reader(file)
            .map_err(|error| format!("O board está corrompido: {error}"))?;
        if board.workspace_id != workspace_id || board.id != board_id {
            return Err("O board não pertence ao workspace informado.".to_owned());
        }
        Ok(board)
    }

    pub fn save_board(&self, mut board: Board) -> Result<Board, String> {
        self.workspace(&board.workspace_id)?;
        validate_board(&board)?;
        board.updated_at = now();
        self.write_board(&board)?;
        let updated_at = board.updated_at;
        self.update_workspace(&board.workspace_id, |workspace| {
            workspace.updated_at = updated_at
        })?;
        Ok(board)
    }

    fn read_index(&self) -> Result<WorkspaceIndex, String> {
        let path = self.root.join("index.json");
        if !path.exists() {
            return Ok(WorkspaceIndex::default());
        }
        let file = fs::File::open(path)
            .map_err(|error| format!("Não foi possível abrir o índice local: {error}"))?;
        serde_json::from_reader(file)
            .map_err(|error| format!("O índice local está corrompido: {error}"))
    }

    fn write_index(&self, index: &WorkspaceIndex) -> Result<(), String> {
        write_json_atomically(&self.root.join("index.json"), index)
    }

    fn write_board(&self, board: &Board) -> Result<(), String> {
        write_json_atomically(&self.board_path(&board.workspace_id, &board.id)?, board)
    }

    fn workspace(&self, id: &str) -> Result<WorkspaceSummary, String> {
        self.read_index()?
            .workspaces
            .into_iter()
            .find(|workspace| workspace.id == id)
            .ok_or_else(|| "Workspace não encontrado.".to_owned())
    }

    fn update_workspace(
        &self,
        id: &str,
        update: impl FnOnce(&mut WorkspaceSummary),
    ) -> Result<(), String> {
        let mut index = self.read_index()?;
        let workspace = index
            .workspaces
            .iter_mut()
            .find(|workspace| workspace.id == id)
            .ok_or_else(|| "Workspace não encontrado.".to_owned())?;
        update(workspace);
        self.write_index(&index)
    }

    fn board_directory(&self, workspace_id: &str) -> PathBuf {
        self.root.join(workspace_id).join("boards")
    }

    fn board_path(&self, workspace_id: &str, board_id: &str) -> Result<PathBuf, String> {
        valid_id(workspace_id)?;
        valid_id(board_id)?;
        Ok(self
            .board_directory(workspace_id)
            .join(format!("{board_id}.json")))
    }
}

fn validate_board(board: &Board) -> Result<(), String> {
    valid_id(&board.id)?;
    valid_id(&board.workspace_id)?;
    validate_name(&board.name)?;
    if board.schema_version != BOARD_SCHEMA_VERSION {
        return Err("A versão do board não é suportada.".to_owned());
    }
    if board
        .element_order
        .iter()
        .any(|id| !board.elements.contains_key(id))
    {
        return Err("A ordem dos elementos referencia itens inexistentes.".to_owned());
    }
    Ok(())
}

pub fn validate_name(name: &str) -> Result<(), String> {
    let length = name.trim().chars().count();
    if !(1..=120).contains(&length) {
        return Err("O nome deve ter entre 1 e 120 caracteres.".to_owned());
    }
    Ok(())
}

fn valid_id(id: &str) -> Result<(), String> {
    if id.is_empty()
        || id.len() > 64
        || !id.chars().all(|character| {
            character.is_ascii_alphanumeric() || character == '-' || character == '_'
        })
    {
        return Err("Identificador inválido.".to_owned());
    }
    Ok(())
}

fn write_json_atomically(path: &Path, value: &impl serde::Serialize) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| "Caminho de armazenamento inválido.".to_owned())?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("Não foi possível criar o diretório de armazenamento: {error}"))?;
    let temporary = path.with_extension("json.tmp");
    let encoded = serde_json::to_vec_pretty(value)
        .map_err(|error| format!("Não foi possível serializar os dados: {error}"))?;
    let mut file = fs::File::create(&temporary)
        .map_err(|error| format!("Não foi possível preparar o salvamento: {error}"))?;
    file.write_all(&encoded)
        .map_err(|error| format!("Não foi possível escrever os dados: {error}"))?;
    file.sync_all()
        .map_err(|error| format!("Não foi possível confirmar os dados no disco: {error}"))?;
    fs::rename(&temporary, path)
        .map_err(|error| format!("Não foi possível concluir o salvamento: {error}"))
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
