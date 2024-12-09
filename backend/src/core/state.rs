//! Tauri App State Management
//!
//! Read more on https://v2.tauri.app/develop/state-management/

use std::sync::{Arc, Mutex};

use crate::{adapters::SysinfoAdapter, cpu::CpuMonitor, memory::MemoryMonitor};

pub type AppState = AppStateBase<SysinfoAdapter, SysinfoAdapter>;

#[derive(Debug)]
pub struct AppStateBase<TCpu, TMemory> {
    pub cpu_monitor: Arc<Mutex<TCpu>>,
    pub memory_monitor: Arc<Mutex<TMemory>>,
}

impl<TCpu, TMemory> AppStateBase<TCpu, TMemory>
where
    TCpu: CpuMonitor,
    TMemory: MemoryMonitor,
{
    pub fn new(cpu_monitor: TCpu, memory_monitor: TMemory) -> AppStateBase<TCpu, TMemory> {
        Self {
            cpu_monitor: Arc::new(Mutex::new(cpu_monitor)),
            memory_monitor: Arc::new(Mutex::new(memory_monitor)),
        }
    }
}

impl Default for AppStateBase<SysinfoAdapter, SysinfoAdapter> {
    fn default() -> Self {
        let system = Arc::new(Mutex::new(SysinfoAdapter::new()));
        AppStateBase {
            cpu_monitor: Arc::clone(&system),
            memory_monitor: system,
        }
    }
}
