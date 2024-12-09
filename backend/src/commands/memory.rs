use tauri::State;

use crate::{
    core::{error::AppError, state::AppState},
    memory::{MemoryMonitor, MemoryUsage},
};

#[tauri::command]
#[specta::specta]
pub fn get_memory_usage(state: State<'_, AppState>) -> Result<MemoryUsage, AppError> {
    let mut memory_monitor = state.memory_monitor.lock().map_err(|_| AppError::Lock)?;
    Ok(memory_monitor.get_memory_usage())
}
