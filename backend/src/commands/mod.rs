pub mod cpu;
pub mod memory;

use specta_typescript::{formatter, BigIntExportBehavior, Typescript};
use tauri_specta::{collect_commands, Builder};
use tracing::debug;

use crate::core::{settings::CONFIG, state::AppStateBase};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let specta_builder = Builder::<tauri::Wry>::new().commands(collect_commands![
        cpu::get_cpu_stats,
        memory::get_memory_usage
    ]);

    #[cfg(debug_assertions)]
    {
        specta_builder
            .export(
                Typescript::default()
                    .bigint(BigIntExportBehavior::Number)
                    .formatter(formatter::prettier),
                &CONFIG.tauri.bindings_output_path,
            )
            .expect("Failed to export typescript bindings");
        debug!(
            "Successfully exported typescript bindings to {}",
            &CONFIG.tauri.bindings_output_path
        )
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppStateBase::default())
        .invoke_handler(specta_builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
