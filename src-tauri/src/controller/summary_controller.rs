use crate::repository::dialy_repository::{Dialy, DialyRepository};
use tauri::State;

#[tauri::command]
pub fn sum_by_date_command(repo: State<'_, DialyRepository>, date: String) -> u32 {
    let records = repo.select(date);
    let mut sum = 0;
    let _ = records.iter().map(|r| sum += r.calorie);
    sum
}
