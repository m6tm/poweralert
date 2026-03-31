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
               
               let position_window = |window: &tauri::WebviewWindow| {
                   if let Ok(Some(monitor)) = window.primary_monitor() {
                       let size = monitor.size();
                       let scale_factor = monitor.scale_factor();
                       let margin = 20.0 * scale_factor;
                       let physical_width = 400.0 * scale_factor;
                       let x = (size.width as f64 - physical_width - margin) as i32;
                       let y = margin as i32;
                       let _ = window.set_position(tauri::PhysicalPosition::new(x, y));
                   }
               };

               if let Some(window) = handle.get_webview_window("notification") {
                   let js_code = format!("window.location.href = '{}';", url);
                   let _ = window.eval(&js_code);
                   position_window(&window);
                   let _ = window.show();
                   let _ = window.set_focus();
               } else {
                   let builder = WebviewWindowBuilder::new(&handle, "notification", WebviewUrl::App(url.into()))
                      .title("PowerAlert - Notification")
                      .inner_size(400.0, 160.0)
                      .always_on_top(true)
                      .decorations(false)
                      .transparent(true);
                      
                   if let Ok(window) = builder.build() {
                       position_window(&window);
                   }
               }
           },
           Err(e) => {
               log::error!("Erreur de parsing de l'alerte: {}. Payload: {}", e, event.payload());
           }
        }
      });

      // Configuration du System Tray avec l'interface web personnalisée (tray.astro)
      let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .on_tray_icon_event(|tray, event| {
           if let TrayIconEvent::Click { position, button, button_state, .. } = event {
              if (button == tauri::tray::MouseButton::Left || button == tauri::tray::MouseButton::Right) && button_state == tauri::tray::MouseButtonState::Up {
                  let app = tray.app_handle();
                  
                  // Fonction pour positionner le tray au-dessus de l'icône de la barre des tâches
                  let position_tray_window = |window: &tauri::WebviewWindow, cursor_pos: tauri::PhysicalPosition<f64>| {
                      if let Ok(Some(monitor)) = window.primary_monitor() {
                          let scale = monitor.scale_factor();
                          let logical_width = 280.0;
                          let logical_height = 210.0; // Ajusté après la suppression de 'Paramètres'
                          
                          let physical_width = logical_width * scale;
                          let physical_height = logical_height * scale;
                          
                          let x = cursor_pos.x - (physical_width / 2.0);
                          let y = cursor_pos.y - physical_height - (10.0 * scale);
                          
                          let _ = window.set_position(tauri::PhysicalPosition::new(x, y));
                      }
                  };

                  if let Some(window) = app.get_webview_window("tray") {
                      if window.is_visible().unwrap_or(false) {
                          let _ = window.hide();
                      } else {
                          position_tray_window(&window, position);
                          let _ = window.show();
                          let _ = window.set_focus();
                      }
                  } else {
                      let builder = WebviewWindowBuilder::new(app, "tray", WebviewUrl::App("/tray".into()))
                          .title("PowerAlert - Tray Menu")
                          .inner_size(280.0, 210.0) // Ajusté après la suppression de 'Paramètres'
                          .decorations(false)
                          .transparent(true)
                          .always_on_top(true)
                          .skip_taskbar(true)
                          .visible(false); // Sera affichée après le calcul de position
                          
                      if let Ok(window) = builder.build() {
                          position_tray_window(&window, position);
                          let _ = window.show();
                          let _ = window.set_focus();
                          
                          // Comportement crucial : masquer le menu personnalisé lorsqu'on clique ailleurs (perte de focus)
                          let w_clone = window.clone();
                          window.on_window_event(move |event| {
                              if let tauri::WindowEvent::Focused(false) = event {
                                  let _ = w_clone.hide();
                              }
                          });
                      }
                  }
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
