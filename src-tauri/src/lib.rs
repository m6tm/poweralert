pub mod domain;
pub mod infrastructure;
pub mod application;

use crate::infrastructure::battery_adapter::BatteryAdapter;
use crate::infrastructure::config_adapter::ConfigAdapter;
use crate::application::battery_use_case::GetBatteryStatusUseCase;
use crate::application::config_use_case::{GetConfigUseCase, SaveConfigUseCase};
use crate::application::monitor_service::BatteryMonitorService;
use crate::domain::config::AppConfig;
use crate::domain::battery_status::BatteryInfo;
use crate::domain::battery_alert::{BatteryAlert, AlertType};
use tauri::{Listener, WebviewUrl, WebviewWindowBuilder, Manager};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri::menu::{Menu, MenuItem};

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

#[tauri::command]
/// Récupère la configuration actuelle de l'application.
fn get_config(app_handle: tauri::AppHandle) -> Result<AppConfig, String> {
    let adapter = ConfigAdapter::new(&app_handle);
    let use_case = GetConfigUseCase::new(adapter);
    use_case.execute()
}

#[tauri::command]
/// Sauvegarde la nouvelle configuration de l'application.
fn save_config(app_handle: tauri::AppHandle, config: AppConfig) -> Result<(), String> {
    let adapter = ConfigAdapter::new(&app_handle);
    let use_case = SaveConfigUseCase::new(adapter);
    use_case.execute(config)
}

#[tauri::command]
/// Ouvre ou restaure la fenêtre principale du tableau de bord.
fn open_dashboard(app_handle: tauri::AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

#[tauri::command]
/// Termine complètement l'application.
fn terminate_app(app_handle: tauri::AppHandle) {
    app_handle.exit(0);
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

      // Initialisation du plugin de lancement automatique (Windows registry)
      app.handle().plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, Some(vec!["--minimized"])))?;
      
      // Initialisation du plugin de persistance de l'état de la fenêtre
      app.handle().plugin(tauri_plugin_window_state::Builder::default().build())?;

      // Démarrage de la surveillance asynchrone de la batterie avec l'adaptateur système
      BatteryMonitorService::start_monitoring(app.handle().clone(), BatteryAdapter::new());
      
      // Écouteur global pour les alertes de batterie afin d'ouvrir la fenêtre de notification
      let handle = app.handle().clone();
      app.listen("battery-alert", move |event| {
        match serde_json::from_str::<BatteryAlert>(event.payload()) {
           Ok(alert) => {
               let type_param = if alert.alert_type == AlertType::DisconnectCharger { "full" } else { "low" };
               
               let handle_clone = handle.clone();
               let config = GetConfigUseCase::new(ConfigAdapter::new(&handle_clone)).execute().unwrap_or_else(|_| AppConfig::default());
               let threshold = if type_param == "full" { config.high_threshold } else { config.low_threshold };

               // Récupération du pourcentage actuel pour l'UI de notification
               let adapter = BatteryAdapter::new();
               let percentage = GetBatteryStatusUseCase::new(adapter).execute()
                  .map(|info| info.percentage)
                  .unwrap_or(0.0);

               let url = format!("/notification?type={}&level={:.0}&threshold={}", type_param, percentage, threshold);
               
               if let Some(window) = handle.get_webview_window("notification") {
                   let js_code = format!("window.location.href = '{}';", url);
                   let _ = window.eval(&js_code);
                   let _ = window.show();
                   let _ = window.set_focus();
               } else {
                   let _ = WebviewWindowBuilder::new(&handle, "notification", WebviewUrl::App(url.into()))
                      .title("PowerAlert - Notification")
                      .inner_size(400.0, 160.0)
                      .always_on_top(true)
                      .decorations(false)
                      .transparent(true)
                      .build();
               }
           },
           Err(e) => {
               log::error!("Erreur de parsing de l'alerte: {}. Payload: {}", e, event.payload());
           }
        }
      });

      // Configuration du System Tray (Icône de notification)
      let quit_i = MenuItem::with_id(app, "quit", "Quitter PowerAlert", true, None::<&str>)?;
      let show_i = MenuItem::with_id(app, "show", "Ouvrir le Tableau de Bord", true, None::<&str>)?;
      let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

      let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(move |app: &tauri::AppHandle, event| {
          match event.id.as_ref() {
            "quit" => {
              app.exit(0);
            }
            "show" => {
              if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.unminimize();
                let _ = window.set_focus();
              }
            }
            _ => {}
          }
        })
        .on_tray_icon_event(|tray, event| {
           if let TrayIconEvent::Click { .. } = event {
              let app = tray.app_handle();
              if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.unminimize();
                let _ = window.set_focus();
              }
           }
        })
        .build(app)?;

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      get_battery_status,
      get_config,
      save_config,
      open_dashboard,
      terminate_app
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
