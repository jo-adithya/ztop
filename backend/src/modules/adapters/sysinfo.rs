use sysinfo::System;

#[derive(Debug)]
pub struct SysinfoAdapter {
    pub system: System,
}

impl SysinfoAdapter {
    /// Creates a new [`SysinfoAdapter`].
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        SysinfoAdapter { system }
    }
}

impl Default for SysinfoAdapter {
    fn default() -> Self {
        Self::new()
    }
}
