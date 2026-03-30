pub mod domain;
pub mod infrastructure;
pub mod application;

use crate::infrastructure::battery_adapter::BatteryAdapter;
use crate::application::battery_use_case::GetBatteryStatusUseCase;
use crate::application::monitor_service::BatteryMonitorService;
use crate::domain::battery_status::BatteryInfo;

#[tauri::command]
/// Récupère l'état actuel de la batterie via le cas d'utilisation GetBatteryStatusUseCase.
///
/// # Retourne
/// * `Ok(BatteryInfo)` - Les informations sur la batterie (charge, état, etc.).
/// * `Err(String)` - Un message d'erreur si la lecture échoue.
fn get_battery_status() -> Result<BatteryInfo, String> {
    let adapter = BatteryAdapter::new();
    let use_case = GetBatteryStatusUseCase::new(adapter);
    use_case.execute()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
/// Point d'entrée principal de l'application Tauri.
/// Configure les plugins, les gestionnaires de commandes et lance la boucle d'événements.
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      // Démarrage de la surveillance asynchrone de la batterie
      BatteryMonitorService::start_monitoring(app.handle().clone());
      
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![get_battery_status])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
