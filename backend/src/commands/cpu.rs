use tauri::State;

use crate::{
    core::{error::AppError, state::AppState},
    cpu::{CpuMonitor, CpuStats},
};

#[tauri::command]
#[specta::specta]
pub fn get_cpu_stats(state: State<'_, AppState>) -> Result<Vec<CpuStats>, AppError> {
    let mut cpu_monitor = state.cpu_monitor.lock().map_err(|_| AppError::Lock)?;
    Ok(cpu_monitor.get_cpu_stats())
}
