use serde::Serialize;
use specta::Type;

#[derive(Debug, Serialize, Type)]
pub struct CpuStats {
    pub name: String,
    pub usage: u8,
    pub frequency: u64,
}

pub trait CpuMonitor {
    fn get_cpu_stats(&mut self) -> Vec<CpuStats>;
}
