use sysinfo::MemoryRefreshKind;

use super::model::{MemoryMonitor, MemoryUsage};
use crate::adapters::SysinfoAdapter;

impl MemoryMonitor for SysinfoAdapter {
    fn get_memory_usage(&mut self) -> MemoryUsage {
        self.system
            .refresh_memory_specifics(MemoryRefreshKind::nothing().with_ram());
        MemoryUsage {
            total: self.system.total_memory(),
            free: self.system.free_memory(),
            available: self.system.available_memory(),
            used: self.system.used_memory(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_memory_must_be_greater_than_the_rest() {
        let mut system = SysinfoAdapter::new();
        let memory_usage = system.get_memory_usage();
        assert!(memory_usage.total > 0);
        assert!(memory_usage.total > memory_usage.free);
        assert!(memory_usage.total > memory_usage.available);
        assert!(memory_usage.total > memory_usage.used);
    }

    #[test]
    fn used_memory_must_be_greater_than_0() {
        let mut system = SysinfoAdapter::new();
        let memory_usage = system.get_memory_usage();
        assert!(memory_usage.used > 0);
    }

    #[test]
    #[cfg(not(target_os = "macos"))]
    fn available_memory_must_be_greater_than_0() {
        let mut system = SysinfoAdapter::new();
        let memory_usage = system.get_memory_usage();
        assert!(memory_usage.available > 0);
    }
}
