use serde::Serialize;
use specta::Type;

#[derive(Debug, Serialize, Type)]
pub struct MemoryUsage {
    pub total: u64,
    pub free: u64,
    pub available: u64,
    pub used: u64,
}

pub trait MemoryMonitor {
    fn get_memory_usage(&mut self) -> MemoryUsage;
}
