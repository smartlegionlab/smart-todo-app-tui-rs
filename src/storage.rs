use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use directories::ProjectDirs;

use crate::app::Todo;

fn data_file() -> Result<PathBuf> {
    let dirs = ProjectDirs::from("com", "smartlegionlab", "smart-todo-app-tui")
        .context("Failed to determine user data directory")?;
    let dir = dirs.data_dir().to_path_buf();
    fs::create_dir_all(&dir).with_context(|| format!("Failed to create {dir:?}"))?;
    Ok(dir.join("todos.json"))
}

pub fn load() -> Result<Vec<Todo>> {
    let path = data_file()?;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = fs::read_to_string(&path).with_context(|| format!("Failed to read {path:?}"))?;
    let todos: Vec<Todo> =
        serde_json::from_str(&data).with_context(|| format!("Invalid JSON in {path:?}"))?;
    Ok(todos)
}

pub fn save(todos: &[Todo]) -> Result<()> {
    let path = data_file()?;
    let data = serde_json::to_string_pretty(todos)?;
    fs::write(&path, data).with_context(|| format!("Failed to write {path:?}"))?;
    Ok(())
}
