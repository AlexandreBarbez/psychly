pub mod db;
pub mod journal;
pub mod therapy;
pub mod analysis;
pub mod export;

use std::sync::Arc;
use tauri::Manager;

use therapy::infrastructure::ollama_client::OllamaClient;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      // Resolve app root for portable paths
      let app_root = app.path().resource_dir()
        .unwrap_or_else(|_| std::env::current_dir().unwrap());
      let database = Arc::new(
        db::Database::open(&app_root)
          .expect("Failed to initialize database"),
      );
      app.manage(database);

      // Initialize Ollama client with Qwen 2.5 14B
      let ollama = OllamaClient::new("qwen2.5:14b-instruct-q5_K_M".to_string());
      app.manage(ollama);

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      journal::application::commands::create_entry,
      journal::application::commands::get_entry,
      journal::application::commands::list_entries,
      journal::application::commands::update_entry,
      journal::application::commands::delete_entry,
      journal::application::commands::search_entries,
      therapy::application::commands::start_chat_session,
      therapy::application::commands::send_message,
      therapy::application::commands::list_chat_sessions,
      therapy::application::commands::get_chat_session,
      therapy::application::commands::check_ollama_status,
      analysis::application::commands::get_entry_analysis,
      export::export_journal,
      export::import_journal,
      export::sqlite::backup_db,
      export::sqlite::restore_db,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
