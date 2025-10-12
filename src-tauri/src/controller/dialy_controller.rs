use crate::repository::dialy_repository::{Dialy, DialyRepository};
use tauri::State;

#[tauri::command]
pub fn insert_command(repo: State<'_, DialyRepository>, dialy: Dialy) {
    repo.insert(dialy);
}

#[tauri::command]
pub fn select_all_command(repo: State<'_, DialyRepository>) -> Vec<Dialy> {
    repo.select_all()
}

#[tauri::command]
pub fn delete_command(repo: State<'_, DialyRepository>, dialy: Dialy) {
    repo.delete(dialy);
}
