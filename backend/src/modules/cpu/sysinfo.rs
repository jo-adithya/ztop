use super::model::{CpuMonitor, CpuStats};
use crate::adapters::SysinfoAdapter;

impl CpuMonitor for SysinfoAdapter {
    fn get_cpu_stats(&mut self) -> Vec<CpuStats> {
        self.system.refresh_cpu_all();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        self.system.refresh_cpu_all();
        self.system
            .cpus()
            .iter()
            .map(|cpu| CpuStats {
                name: cpu.name().to_string(),
                usage: cpu.cpu_usage().round() as u8,
                frequency: cpu.frequency(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_the_stats_of_every_cpu_in_the_system() {
        let mut sys = SysinfoAdapter::new();
        let cpu_stats = sys.get_cpu_stats();
        assert_eq!(cpu_stats.len(), sys.system.cpus().len());
    }

    #[test]
    fn cpu_usage_must_be_between_0_and_100() {
        let mut sys = SysinfoAdapter::new();
        let cpu_stats = sys.get_cpu_stats();
        for cpu in cpu_stats {
            assert!(cpu.usage <= 100);
        }
    }

    #[test]
    fn cpu_name_must_not_be_empty() {
        let mut sys = SysinfoAdapter::new();
        let cpu_stats = sys.get_cpu_stats();
        for cpu in cpu_stats {
            assert_ne!(cpu.name, "");
        }
    }
}
