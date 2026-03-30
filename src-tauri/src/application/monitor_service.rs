use std::time::Duration;
use crate::infrastructure::battery_adapter::BatteryAdapter;
use crate::application::battery_use_case::GetBatteryStatusUseCase;
use crate::domain::battery_alert::{AlertService, DEFAULT_LOW_THRESHOLD, DEFAULT_HIGH_THRESHOLD};
use log::{info, warn};
use tauri::AppHandle;

/// Service responsable de la surveillance périodique de l'état de la batterie.
pub struct BatteryMonitorService;

impl BatteryMonitorService {
    /// Démarre la boucle de surveillance asynchrone.
    ///
    /// # Arguments
    /// * `_app_handle` - Le handle de l'application Tauri pour émettre des événements ou accéder à l'état.
    pub fn start_monitoring(_app_handle: AppHandle) {
        tokio::spawn(async move {
            let adapter = BatteryAdapter::new();
            let get_status_use_case = GetBatteryStatusUseCase::new(adapter);
            let mut interval = tokio::time::interval(Duration::from_secs(60));

            info!("Démarrage du cycle de surveillance de la batterie (60s).");

            loop {
                interval.tick().await;

                match get_status_use_case.execute() {
                    Ok(info) => {
                        info!("Niveau de batterie : {:.0}% (Chargement: {})", info.percentage, info.is_charging);
                        
                        // Vérification des alertes
                        if let Some(alert) = AlertService::check_for_alerts(&info, DEFAULT_LOW_THRESHOLD, DEFAULT_HIGH_THRESHOLD) {
                            warn!("Alerte détectée : {:?}", alert.alert_type);
                            // TODO: Émettre un événement vers le frontend ou afficher une notification
                            // Pour [TICKET-2.3], on se contente de logger.
                        }
                    }
                    Err(e) => {
                        warn!("Erreur lors de la récupération de l'état de la batterie : {}", e);
                    }
                }
            }
        });
    }
}
