// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ztop_lib::{
    adapters::SysinfoAdapter, commands, core::init, cpu::CpuMonitor, memory::MemoryMonitor,
};

fn main() {
    init::init_tracing();

    // let mut sys = SysinfoAdapter::new();
    // let cpu_usage = sys.get_cpu_stats();
    // for cpu in &cpu_usage {
    //     tracing::info!("{}: {}%   {}Hz", cpu.name, cpu.usage, cpu.frequency);
    // }
    //
    // let memory_usage = sys.get_memory_usage();
    // tracing::info!(
    //     "\nMemory Usage:\n\
    //                 ------------\n\
    //                 Total Memory: {}\n\
    //                 Used Memory: {}\n\
    //                 Available Memory: {}\n\
    //                 Free Memory: {}\n",
    //     memory_usage.total,
    //     memory_usage.used,
    //     memory_usage.available,
    //     memory_usage.free
    // );

    commands::run()
}
