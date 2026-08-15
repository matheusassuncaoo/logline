use crate::domain::{
    AssetData, Board, BoardSummary, WorkspaceIndex, WorkspaceSummary, BOARD_SCHEMA_VERSION,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{Cursor, Read, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::{AppHandle, Manager};
use zip::{write::SimpleFileOptions, ZipArchive, ZipWriter};

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
        let persistence = Self { root };
        persistence.recover_journals()?;
        Ok(persistence)
    }

    pub fn list_workspaces(&self) -> Result<Vec<WorkspaceSummary>, String> {
        self.read_index()?
            .workspaces
            .into_iter()
            .map(|mut workspace| {
                workspace.board_count = self.board_count(&workspace.id)?;
                Ok(workspace)
            })
            .collect()
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
        fs::create_dir_all(self.asset_directory(&workspace.id))
            .map_err(|error| format!("Não foi possível preparar os assets: {error}"))?;

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
        // The board file is durable before the index is updated. Listing workspaces
        // reconciles the count from disk if the second write is interrupted.
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
        boards.sort_by_key(|board| std::cmp::Reverse(board.updated_at));
        Ok(boards)
    }

    pub fn open_board(&self, workspace_id: &str, board_id: &str) -> Result<Board, String> {
        self.workspace(workspace_id)?;
        let path = self.board_path(workspace_id, board_id)?;
        let file = fs::File::open(path)
            .map_err(|error| format!("Não foi possível abrir o board: {error}"))?;
        let mut board: Board = serde_json::from_reader(file)
            .map_err(|error| format!("O board está corrompido: {error}"))?;
        if board.workspace_id != workspace_id || board.id != board_id {
            return Err("O board não pertence ao workspace informado.".to_owned());
        }
        if migrate_board(&mut board)? {
            self.write_board(&board)?;
        }
        Ok(board)
    }

    pub fn save_board(&self, mut board: Board) -> Result<Board, String> {
        let existing = self.open_board(&board.workspace_id, &board.id)?;
        validate_board(&board)?;
        board.created_at = existing.created_at;
        board.updated_at = now();
        write_json_atomically(&self.journal_path(&board.workspace_id, &board.id)?, &board)?;
        self.write_board(&board)?;
        let _ = fs::remove_file(self.journal_path(&board.workspace_id, &board.id)?);
        let updated_at = board.updated_at;
        self.update_workspace(&board.workspace_id, |workspace| {
            workspace.updated_at = updated_at
        })?;
        Ok(board)
    }

    pub fn add_asset(
        &self,
        workspace_id: &str,
        file_name: &str,
        mime_type: &str,
        bytes: &[u8],
    ) -> Result<AssetData, String> {
        self.workspace(workspace_id)?;
        if bytes.is_empty() || bytes.len() > 25 * 1024 * 1024 || !mime_type.starts_with("image/") {
            return Err("O arquivo deve ser uma imagem de até 25 MB.".to_owned());
        }
        let id = format!("{:x}", Sha256::digest(bytes));
        let extension = safe_extension(file_name, mime_type);
        let path = self
            .asset_directory(workspace_id)
            .join(format!("{id}.{extension}"));
        if !path.exists() {
            write_bytes_atomically(&path, bytes)?;
        }
        Ok(AssetData {
            id,
            data_url: format!("data:{mime_type};base64,{}", BASE64.encode(bytes)),
        })
    }

    pub fn read_asset(&self, workspace_id: &str, asset_id: &str) -> Result<AssetData, String> {
        self.workspace(workspace_id)?;
        valid_id(asset_id)?;
        let path = fs::read_dir(self.asset_directory(workspace_id))
            .map_err(|error| format!("Não foi possível listar os assets: {error}"))?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .find(|path| path.file_stem().and_then(|stem| stem.to_str()) == Some(asset_id))
            .ok_or_else(|| "Imagem não encontrada.".to_owned())?;
        let bytes =
            fs::read(&path).map_err(|error| format!("Não foi possível abrir a imagem: {error}"))?;
        let mime_type = mime_for_extension(
            path.extension()
                .and_then(|extension| extension.to_str())
                .unwrap_or(""),
        );
        Ok(AssetData {
            id: asset_id.to_owned(),
            data_url: format!("data:{mime_type};base64,{}", BASE64.encode(bytes)),
        })
    }

    pub fn export_workspace(&self, workspace_id: &str) -> Result<Vec<u8>, String> {
        let workspace = self.workspace(workspace_id)?;
        let mut archive = ZipWriter::new(Cursor::new(Vec::new()));
        archive
            .start_file("manifest.json", SimpleFileOptions::default())
            .map_err(|error| format!("Não foi possível criar o arquivo portátil: {error}"))?;
        archive
            .write_all(
                &serde_json::to_vec(&workspace)
                    .map_err(|error| format!("Não foi possível serializar o workspace: {error}"))?,
            )
            .map_err(|error| format!("Não foi possível criar o arquivo portátil: {error}"))?;
        for directory in ["boards", "assets"] {
            let path = self.workspace_directory(workspace_id).join(directory);
            if !path.exists() {
                continue;
            }
            for entry in fs::read_dir(path)
                .map_err(|error| format!("Não foi possível ler o workspace: {error}"))?
            {
                let entry =
                    entry.map_err(|error| format!("Não foi possível ler o workspace: {error}"))?;
                if !entry
                    .file_type()
                    .map_err(|error| format!("Não foi possível ler o workspace: {error}"))?
                    .is_file()
                {
                    continue;
                }
                archive
                    .start_file(
                        format!("{directory}/{}", entry.file_name().to_string_lossy()),
                        SimpleFileOptions::default(),
                    )
                    .map_err(|error| {
                        format!("Não foi possível criar o arquivo portátil: {error}")
                    })?;
                archive
                    .write_all(
                        &fs::read(entry.path()).map_err(|error| {
                            format!("Não foi possível ler o workspace: {error}")
                        })?,
                    )
                    .map_err(|error| {
                        format!("Não foi possível criar o arquivo portátil: {error}")
                    })?;
            }
        }
        let cursor = archive
            .finish()
            .map_err(|error| format!("Não foi possível finalizar o arquivo portátil: {error}"))?;
        Ok(cursor.into_inner())
    }

    pub fn import_workspace(&self, bytes: &[u8]) -> Result<WorkspaceSummary, String> {
        if bytes.is_empty() || bytes.len() > 100 * 1024 * 1024 {
            return Err("O arquivo .logline é inválido ou excede 100 MB.".to_owned());
        }
        let mut archive = ZipArchive::new(Cursor::new(bytes))
            .map_err(|error| format!("Não foi possível abrir o arquivo .logline: {error}"))?;
        let mut manifest = String::new();
        archive
            .by_name("manifest.json")
            .map_err(|_| "O arquivo .logline não contém um manifesto.".to_owned())?
            .read_to_string(&mut manifest)
            .map_err(|error| format!("Não foi possível ler o manifesto: {error}"))?;
        let exported: WorkspaceSummary = serde_json::from_str(&manifest)
            .map_err(|error| format!("O manifesto é inválido: {error}"))?;
        validate_name(&exported.name)?;
        let workspace = self.create_workspace(exported.name)?;
        for index in 0..archive.len() {
            let mut entry = archive
                .by_index(index)
                .map_err(|error| format!("Não foi possível importar o arquivo: {error}"))?;
            let name = entry.name().replace('\\', "/");
            if !(name.starts_with("boards/") || name.starts_with("assets/"))
                || name.contains("..")
                || name.ends_with('/')
            {
                continue;
            }
            let file_name = name.rsplit('/').next().unwrap_or_default();
            if file_name.is_empty() {
                continue;
            }
            let mut contents = Vec::new();
            entry
                .read_to_end(&mut contents)
                .map_err(|error| format!("Não foi possível importar o arquivo: {error}"))?;
            if name.starts_with("boards/") {
                let mut board: Board = serde_json::from_slice(&contents)
                    .map_err(|error| format!("Um board importado é inválido: {error}"))?;
                board.workspace_id = workspace.id.clone();
                migrate_board(&mut board)?;
                validate_board(&board)?;
                self.write_board(&board)?;
            } else {
                write_bytes_atomically(
                    &self.asset_directory(&workspace.id).join(file_name),
                    &contents,
                )?;
            }
        }
        let count = self.list_boards(&workspace.id)?.len() as u32;
        self.update_workspace(&workspace.id, |item| item.board_count = count)?;
        self.workspace(&workspace.id)
    }

    fn read_index(&self) -> Result<WorkspaceIndex, String> {
        let path = self.root.join("index.json");
        if !path.exists() {
            return Ok(WorkspaceIndex::default());
        }
        let file = fs::File::open(path)
            .map_err(|error| format!("Não foi possível abrir o índice local: {error}"))?;
        let index: WorkspaceIndex = serde_json::from_reader(file)
            .map_err(|error| format!("O índice local está corrompido: {error}"))?;
        validate_workspace_index(&index)?;
        Ok(index)
    }

    fn write_index(&self, index: &WorkspaceIndex) -> Result<(), String> {
        write_json_atomically(&self.root.join("index.json"), index)
    }

    fn write_board(&self, board: &Board) -> Result<(), String> {
        write_json_atomically(&self.board_path(&board.workspace_id, &board.id)?, board)
    }

    fn workspace(&self, id: &str) -> Result<WorkspaceSummary, String> {
        valid_id(id)?;
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

    fn board_count(&self, workspace_id: &str) -> Result<u32, String> {
        let directory = self.board_directory(workspace_id);
        let entries = fs::read_dir(directory)
            .map_err(|error| format!("Não foi possível listar os boards: {error}"))?;
        let count = entries
            .filter_map(Result::ok)
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .and_then(|extension| extension.to_str())
                    == Some("json")
            })
            .count();
        u32::try_from(count).map_err(|_| "O workspace possui boards demais.".to_owned())
    }

    fn board_directory(&self, workspace_id: &str) -> PathBuf {
        self.workspace_directory(workspace_id).join("boards")
    }

    fn asset_directory(&self, workspace_id: &str) -> PathBuf {
        self.workspace_directory(workspace_id).join("assets")
    }

    fn workspace_directory(&self, workspace_id: &str) -> PathBuf {
        self.root.join(workspace_id)
    }

    fn journal_path(&self, workspace_id: &str, board_id: &str) -> Result<PathBuf, String> {
        valid_id(workspace_id)?;
        valid_id(board_id)?;
        Ok(self
            .workspace_directory(workspace_id)
            .join("journal")
            .join(format!("{board_id}.json")))
    }

    fn board_path(&self, workspace_id: &str, board_id: &str) -> Result<PathBuf, String> {
        valid_id(workspace_id)?;
        valid_id(board_id)?;
        Ok(self
            .board_directory(workspace_id)
            .join(format!("{board_id}.json")))
    }

    fn recover_journals(&self) -> Result<(), String> {
        for workspace in fs::read_dir(&self.root)
            .map_err(|error| format!("Não foi possível recuperar os boards: {error}"))?
        {
            let workspace = workspace
                .map_err(|error| format!("Não foi possível recuperar os boards: {error}"))?;
            let journal = workspace.path().join("journal");
            if !journal.is_dir() {
                continue;
            }
            for entry in fs::read_dir(&journal)
                .map_err(|error| format!("Não foi possível recuperar os boards: {error}"))?
            {
                let entry = entry
                    .map_err(|error| format!("Não foi possível recuperar os boards: {error}"))?;
                let file = fs::File::open(entry.path())
                    .map_err(|error| format!("Não foi possível recuperar um board: {error}"))?;
                let mut board: Board = serde_json::from_reader(file)
                    .map_err(|error| format!("O journal de um board está corrompido: {error}"))?;
                migrate_board(&mut board)?;
                self.write_board(&board)?;
                fs::remove_file(entry.path())
                    .map_err(|error| format!("Não foi possível concluir a recuperação: {error}"))?;
            }
        }
        Ok(())
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

fn validate_workspace_index(index: &WorkspaceIndex) -> Result<(), String> {
    if index.schema_version != crate::domain::WORKSPACE_SCHEMA_VERSION {
        return Err("A versão do índice de workspaces não é suportada.".to_owned());
    }
    for workspace in &index.workspaces {
        valid_id(&workspace.id)?;
        validate_name(&workspace.name)?;
    }
    Ok(())
}

fn migrate_board(board: &mut Board) -> Result<bool, String> {
    if board.schema_version > BOARD_SCHEMA_VERSION {
        return Err("A versão do board não é suportada.".to_owned());
    }
    if board.schema_version == BOARD_SCHEMA_VERSION {
        return Ok(false);
    }
    board.schema_version = BOARD_SCHEMA_VERSION;
    Ok(true)
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

fn write_bytes_atomically(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| "Caminho de armazenamento inválido.".to_owned())?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("Não foi possível criar o diretório de armazenamento: {error}"))?;
    let temporary = path.with_extension("tmp");
    let mut file = fs::File::create(&temporary)
        .map_err(|error| format!("Não foi possível preparar o salvamento: {error}"))?;
    file.write_all(bytes)
        .map_err(|error| format!("Não foi possível escrever os dados: {error}"))?;
    file.sync_all()
        .map_err(|error| format!("Não foi possível confirmar os dados no disco: {error}"))?;
    fs::rename(&temporary, path)
        .map_err(|error| format!("Não foi possível concluir o salvamento: {error}"))
}

fn safe_extension(file_name: &str, mime_type: &str) -> String {
    let from_name = Path::new(file_name)
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or("");
    if !from_name.is_empty()
        && from_name.len() <= 8
        && from_name
            .chars()
            .all(|character| character.is_ascii_alphanumeric())
    {
        return from_name.to_ascii_lowercase();
    }
    match mime_type {
        "image/png" => "png",
        "image/jpeg" => "jpg",
        "image/gif" => "gif",
        "image/webp" => "webp",
        "image/svg+xml" => "svg",
        _ => "img",
    }
    .to_owned()
}

fn mime_for_extension(extension: &str) -> &'static str {
    match extension.to_ascii_lowercase().as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        _ => "application/octet-stream",
    }
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_persistence() -> (Persistence, PathBuf) {
        let root = std::env::temp_dir().join(format!("logline-persistence-{}", nanoid::nanoid!()));
        fs::create_dir_all(&root).expect("create test directory");
        (Persistence { root: root.clone() }, root)
    }

    #[test]
    fn saves_an_existing_board_and_reconciles_its_count() {
        let (persistence, root) = test_persistence();
        let workspace = persistence
            .create_workspace("Workspace".to_owned())
            .unwrap();
        let mut board = persistence
            .create_board(&workspace.id, "Board".to_owned())
            .unwrap();

        board.name = "Renamed board".to_owned();
        let saved = persistence.save_board(board).unwrap();
        let workspaces = persistence.list_workspaces().unwrap();

        assert_eq!(saved.name, "Renamed board");
        assert_eq!(workspaces[0].board_count, 1);
        assert_eq!(
            persistence
                .open_board(&workspace.id, &saved.id)
                .unwrap()
                .name,
            saved.name
        );
        fs::remove_dir_all(root).expect("remove test directory");
    }

    #[test]
    fn rejects_saving_a_board_that_was_not_created() {
        let (persistence, root) = test_persistence();
        let workspace = persistence
            .create_workspace("Workspace".to_owned())
            .unwrap();
        let board = Board {
            id: "missing-board".to_owned(),
            workspace_id: workspace.id.clone(),
            name: "Missing".to_owned(),
            schema_version: BOARD_SCHEMA_VERSION,
            created_at: 0,
            updated_at: 0,
            elements: Default::default(),
            element_order: Vec::new(),
        };

        assert!(persistence.save_board(board).is_err());
        assert_eq!(persistence.list_workspaces().unwrap()[0].board_count, 0);
        fs::remove_dir_all(root).expect("remove test directory");
    }

    #[test]
    fn rejects_an_unsupported_workspace_index_schema() {
        let (persistence, root) = test_persistence();
        let index = WorkspaceIndex {
            schema_version: crate::domain::WORKSPACE_SCHEMA_VERSION + 1,
            workspaces: Vec::new(),
        };
        write_json_atomically(&root.join("index.json"), &index).unwrap();

        assert!(persistence.list_workspaces().is_err());
        fs::remove_dir_all(root).expect("remove test directory");
    }
}
