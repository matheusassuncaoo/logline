use crate::domain::{
    AppPreferences, AssetData, Board, BoardSummary, WorkspaceIndex, WorkspaceSummary,
    BOARD_SCHEMA_VERSION,
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

    pub fn get_preferences(&self) -> Result<AppPreferences, String> {
        let path = self.preferences_path()?;
        if !path.exists() {
            return Ok(AppPreferences::default());
        }
        let file = fs::File::open(path)
            .map_err(|error| format!("Não foi possível abrir as configurações: {error}"))?;
        let preferences: AppPreferences = serde_json::from_reader(file)
            .map_err(|error| format!("As configurações locais são inválidas: {error}"))?;
        validate_theme(&preferences.theme)?;
        Ok(preferences)
    }

    pub fn save_preferences(&self, preferences: AppPreferences) -> Result<AppPreferences, String> {
        validate_theme(&preferences.theme)?;
        write_json_atomically(&self.preferences_path()?, &preferences)?;
        Ok(preferences)
    }

    pub fn create_workspace(&self, name: String) -> Result<WorkspaceSummary, String> {
        validate_name(&name)?;
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
        validate_name(&name)?;
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

    pub fn rename_board(
        &self,
        workspace_id: &str,
        board_id: &str,
        name: String,
    ) -> Result<Board, String> {
        validate_name(&name)?;
        let mut board = self.open_board(workspace_id, board_id)?;
        board.name = name;
        self.save_board(board)
    }

    pub fn duplicate_board(
        &self,
        workspace_id: &str,
        board_id: &str,
        name: String,
    ) -> Result<Board, String> {
        validate_name(&name)?;
        let source = self.open_board(workspace_id, board_id)?;
        let now = now();
        let board = Board {
            id: nanoid::nanoid!(),
            workspace_id: workspace_id.to_owned(),
            name,
            schema_version: BOARD_SCHEMA_VERSION,
            created_at: now,
            updated_at: now,
            elements: source.elements,
            element_order: source.element_order,
        };
        self.write_board(&board)?;
        self.update_workspace(workspace_id, |workspace| {
            workspace.board_count += 1;
            workspace.updated_at = now;
        })?;
        Ok(board)
    }

    pub fn delete_board(&self, workspace_id: &str, board_id: &str) -> Result<(), String> {
        self.open_board(workspace_id, board_id)?;
        let path = self.board_path(workspace_id, board_id)?;
        fs::remove_file(path)
            .map_err(|error| format!("Não foi possível remover o board: {error}"))?;
        let updated_at = now();
        self.update_workspace(workspace_id, |workspace| {
            workspace.board_count = workspace.board_count.saturating_sub(1);
            workspace.updated_at = updated_at;
        })
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
        let migrated = migrate_board(&mut board)?;
        let normalized = normalize_element_order(&mut board);
        validate_board(&board)?;
        if migrated || normalized {
            self.write_board(&board)?;
        }
        Ok(board)
    }

    pub fn save_board(&self, mut board: Board) -> Result<Board, String> {
        let existing = self.open_board(&board.workspace_id, &board.id)?;
        normalize_element_order(&mut board);
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
        _file_name: &str,
        mime_type: &str,
        bytes: &[u8],
    ) -> Result<AssetData, String> {
        self.workspace(workspace_id)?;
        if bytes.is_empty() || bytes.len() > 25 * 1024 * 1024 {
            return Err("O arquivo deve ser uma imagem de até 25 MB.".to_owned());
        }
        let extension = image_extension(mime_type)
            .ok_or_else(|| "O formato da imagem não é suportado.".to_owned())?;
        let id = format!("{:x}", Sha256::digest(bytes));
        let directory = self.asset_directory(workspace_id);
        let path = fs::read_dir(&directory)
            .map_err(|error| format!("Não foi possível listar os assets: {error}"))?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .find(|path| path.file_stem().and_then(|stem| stem.to_str()) == Some(id.as_str()))
            .unwrap_or_else(|| directory.join(format!("{id}.{extension}")));
        if !path.exists() {
            write_bytes_atomically(&path, bytes)?;
        }
        let stored_mime_type = mime_for_extension(
            path.extension()
                .and_then(|extension| extension.to_str())
                .unwrap_or(""),
        );
        Ok(AssetData {
            id,
            data_url: format!("data:{stored_mime_type};base64,{}", BASE64.encode(bytes)),
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
        let mut manifest_entry = archive
            .by_name("manifest.json")
            .map_err(|_| "O arquivo .logline não contém um manifesto.".to_owned())?;
        if manifest_entry.size() > 1024 * 1024 {
            return Err("O manifesto do arquivo .logline excede o limite permitido.".to_owned());
        }
        manifest_entry
            .read_to_string(&mut manifest)
            .map_err(|error| format!("Não foi possível ler o manifesto: {error}"))?;
        drop(manifest_entry);
        let exported: WorkspaceSummary = serde_json::from_str(&manifest)
            .map_err(|error| format!("O manifesto é inválido: {error}"))?;
        validate_name(&exported.name)?;
        let workspace = self.create_workspace(exported.name)?;
        let mut imported_size = 0_u64;
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
            if entry.size() > 25 * 1024 * 1024
                || imported_size.saturating_add(entry.size()) > 100 * 1024 * 1024
            {
                return Err("O arquivo .logline excede o limite de conteúdo permitido.".to_owned());
            }
            let mut contents = Vec::new();
            entry
                .read_to_end(&mut contents)
                .map_err(|error| format!("Não foi possível importar o arquivo: {error}"))?;
            imported_size += contents.len() as u64;
            if name.starts_with("boards/") {
                if !file_name.ends_with(".json") {
                    return Err("O arquivo .logline contém um board inválido.".to_owned());
                }
                let mut board: Board = serde_json::from_slice(&contents)
                    .map_err(|error| format!("Um board importado é inválido: {error}"))?;
                if file_name.trim_end_matches(".json") != board.id {
                    return Err(
                        "O arquivo do board não corresponde ao seu identificador.".to_owned()
                    );
                }
                board.workspace_id = workspace.id.clone();
                migrate_board(&mut board)?;
                normalize_element_order(&mut board);
                validate_board(&board)?;
                self.write_board(&board)?;
            } else {
                if !valid_asset_file_name(file_name) {
                    return Err("O arquivo .logline contém um asset inválido.".to_owned());
                }
                let asset_id = file_name
                    .rsplit_once('.')
                    .map(|(id, _)| id)
                    .unwrap_or_default();
                if asset_id != format!("{:x}", Sha256::digest(&contents)) {
                    return Err(
                        "O asset importado não corresponde ao seu identificador.".to_owned()
                    );
                }
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

    fn preferences_path(&self) -> Result<PathBuf, String> {
        Ok(self
            .root
            .parent()
            .ok_or_else(|| "Diretório de configurações inválido.".to_owned())?
            .join("preferences.json"))
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
            if !workspace
                .file_type()
                .map_err(|error| format!("Não foi possível recuperar os boards: {error}"))?
                .is_dir()
            {
                continue;
            }
            let workspace_id = workspace.file_name().to_string_lossy().to_string();
            valid_id(&workspace_id)?;
            let journal = workspace.path().join("journal");
            if !journal.is_dir() {
                continue;
            }
            for entry in fs::read_dir(&journal)
                .map_err(|error| format!("Não foi possível recuperar os boards: {error}"))?
            {
                let entry = entry
                    .map_err(|error| format!("Não foi possível recuperar os boards: {error}"))?;
                if !entry
                    .file_type()
                    .map_err(|error| format!("Não foi possível recuperar os boards: {error}"))?
                    .is_file()
                    || entry
                        .path()
                        .extension()
                        .and_then(|extension| extension.to_str())
                        != Some("json")
                {
                    continue;
                }
                let board_id = entry
                    .path()
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .ok_or_else(|| "O journal de um board é inválido.".to_owned())?
                    .to_owned();
                valid_id(&board_id)?;
                let file = fs::File::open(entry.path())
                    .map_err(|error| format!("Não foi possível recuperar um board: {error}"))?;
                let mut board: Board = serde_json::from_reader(file)
                    .map_err(|error| format!("O journal de um board está corrompido: {error}"))?;
                migrate_board(&mut board)?;
                normalize_element_order(&mut board);
                if board.workspace_id != workspace_id || board.id != board_id {
                    return Err("O journal não corresponde ao board informado.".to_owned());
                }
                validate_board(&board)?;
                self.write_board(&board)?;
                self.update_workspace(&workspace_id, |workspace| {
                    workspace.updated_at = board.updated_at;
                })?;
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
    if board.element_order.len() != board.elements.len()
        || board
            .element_order
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
            != board.element_order.len()
    {
        return Err("A ordem dos elementos deve conter cada item uma única vez.".to_owned());
    }
    for (id, element) in &board.elements {
        valid_id(id)?;
        if element.id != *id {
            return Err("O identificador do elemento não corresponde à sua chave.".to_owned());
        }
        valid_id(&element.id)?;
        if let Some(group_id) = &element.group_id {
            valid_id(group_id)?;
        }
        if !element.x.is_finite()
            || !element.y.is_finite()
            || !element.width.is_finite()
            || !element.height.is_finite()
            || !element.rotation.is_finite()
        {
            return Err("As coordenadas do elemento são inválidas.".to_owned());
        }
    }
    Ok(())
}

fn normalize_element_order(board: &mut Board) -> bool {
    let mut seen = std::collections::HashSet::new();
    let mut element_order = Vec::with_capacity(board.elements.len());
    for id in &board.element_order {
        if board.elements.contains_key(id) && seen.insert(id.clone()) {
            element_order.push(id.clone());
        }
    }
    for id in board.elements.keys() {
        if seen.insert(id.clone()) {
            element_order.push(id.clone());
        }
    }
    if element_order == board.element_order {
        false
    } else {
        board.element_order = element_order;
        true
    }
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

fn validate_theme(theme: &str) -> Result<(), String> {
    if matches!(theme, "system" | "light" | "dark") {
        Ok(())
    } else {
        Err("O tema informado não é suportado.".to_owned())
    }
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

fn image_extension(mime_type: &str) -> Option<&'static str> {
    match mime_type {
        "image/png" => Some("png"),
        "image/jpeg" => Some("jpg"),
        "image/gif" => Some("gif"),
        "image/webp" => Some("webp"),
        "image/svg+xml" => Some("svg"),
        _ => None,
    }
}

fn valid_asset_file_name(file_name: &str) -> bool {
    let Some((id, extension)) = file_name.rsplit_once('.') else {
        return false;
    };
    valid_id(id).is_ok() && matches!(extension, "png" | "jpg" | "jpeg" | "gif" | "webp" | "svg")
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
    use crate::domain::{CanvasElement, CanvasElementKind};

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
    fn renames_duplicates_and_deletes_a_board() {
        let (persistence, root) = test_persistence();
        let workspace = persistence
            .create_workspace("Workspace".to_owned())
            .unwrap();
        let board = persistence
            .create_board(&workspace.id, "Original".to_owned())
            .unwrap();

        let renamed = persistence
            .rename_board(&workspace.id, &board.id, "Renamed".to_owned())
            .unwrap();
        let duplicate = persistence
            .duplicate_board(&workspace.id, &renamed.id, "Copy".to_owned())
            .unwrap();

        assert_eq!(renamed.name, "Renamed");
        assert_ne!(duplicate.id, renamed.id);
        assert_eq!(duplicate.elements.len(), renamed.elements.len());
        assert_eq!(duplicate.element_order, renamed.element_order);
        assert_eq!(persistence.list_workspaces().unwrap()[0].board_count, 2);

        persistence
            .delete_board(&workspace.id, &renamed.id)
            .unwrap();
        assert_eq!(persistence.list_boards(&workspace.id).unwrap().len(), 1);
        assert_eq!(persistence.list_workspaces().unwrap()[0].board_count, 1);
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

    #[test]
    fn deduplicates_assets_and_preserves_them_in_a_portable_workspace() {
        let (source, source_root) = test_persistence();
        let workspace = source.create_workspace("Workspace".to_owned()).unwrap();
        source
            .create_board(&workspace.id, "Board".to_owned())
            .unwrap();
        let bytes = [137, 80, 78, 71];
        let first = source
            .add_asset(&workspace.id, "first.png", "image/png", &bytes)
            .unwrap();
        let second = source
            .add_asset(&workspace.id, "renamed.jpg", "image/png", &bytes)
            .unwrap();

        assert_eq!(first.id, second.id);
        assert_eq!(
            fs::read_dir(source.asset_directory(&workspace.id))
                .unwrap()
                .count(),
            1
        );

        let archive = source.export_workspace(&workspace.id).unwrap();
        let (destination, destination_root) = test_persistence();
        let imported = destination.import_workspace(&archive).unwrap();

        assert_eq!(destination.list_boards(&imported.id).unwrap().len(), 1);
        assert_eq!(
            destination
                .read_asset(&imported.id, &first.id)
                .unwrap()
                .data_url,
            first.data_url
        );
        fs::remove_dir_all(source_root).expect("remove source test directory");
        fs::remove_dir_all(destination_root).expect("remove destination test directory");
    }

    #[test]
    fn recovers_a_journal_and_migrates_a_legacy_board() {
        let (persistence, root) = test_persistence();
        let workspace = persistence
            .create_workspace("Workspace".to_owned())
            .unwrap();
        let mut board = persistence
            .create_board(&workspace.id, "Board".to_owned())
            .unwrap();
        board.schema_version = 0;
        persistence.write_board(&board).unwrap();

        let migrated = persistence.open_board(&workspace.id, &board.id).unwrap();
        assert_eq!(migrated.schema_version, BOARD_SCHEMA_VERSION);

        let mut recovered = migrated.clone();
        recovered.name = "Recovered".to_owned();
        write_json_atomically(
            &persistence
                .journal_path(&workspace.id, &recovered.id)
                .unwrap(),
            &recovered,
        )
        .unwrap();
        persistence.recover_journals().unwrap();

        assert_eq!(
            persistence
                .open_board(&workspace.id, &recovered.id)
                .unwrap()
                .name,
            "Recovered"
        );
        assert!(!persistence
            .journal_path(&workspace.id, &recovered.id)
            .unwrap()
            .exists());
        fs::remove_dir_all(root).expect("remove test directory");
    }

    #[test]
    fn normalizes_legacy_element_order_when_opening_a_board() {
        let (persistence, root) = test_persistence();
        let workspace = persistence
            .create_workspace("Workspace".to_owned())
            .unwrap();
        let mut board = persistence
            .create_board(&workspace.id, "Board".to_owned())
            .unwrap();
        for id in ["b", "a"] {
            board.elements.insert(
                id.to_owned(),
                CanvasElement {
                    id: id.to_owned(),
                    kind: CanvasElementKind::Shape,
                    x: 0.0,
                    y: 0.0,
                    width: 100.0,
                    height: 80.0,
                    rotation: 0.0,
                    content: String::new(),
                    color: "#dff2e9".to_owned(),
                    group_id: None,
                },
            );
        }
        board.element_order = vec!["b".to_owned(), "removed".to_owned(), "b".to_owned()];
        persistence.write_board(&board).unwrap();

        let normalized = persistence.open_board(&workspace.id, &board.id).unwrap();

        assert_eq!(normalized.element_order, vec!["b", "a"]);
        assert_eq!(
            persistence
                .open_board(&workspace.id, &board.id)
                .unwrap()
                .element_order,
            vec!["b", "a"]
        );
        fs::remove_dir_all(root).expect("remove test directory");
    }
}
