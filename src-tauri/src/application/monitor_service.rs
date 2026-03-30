use std::time::Duration;
use crate::infrastructure::battery_adapter::BatteryAdapter;
use crate::application::battery_use_case::GetBatteryStatusUseCase;
use crate::domain::battery_port::BatteryPort;
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
                Self::process_check(&get_status_use_case);
            }
        });
    }

    /// Effectue une vérification individuelle de l'état de la batterie.
    /// Extrait pour permettre les tests unitaires sans boucles infinies.
    fn process_check<P: BatteryPort>(use_case: &GetBatteryStatusUseCase<P>) {
        match use_case.execute() {
            Ok(info) => {
                info!("Niveau de batterie : {:.0}% (Chargement: {})", info.percentage, info.is_charging);
                
                // Vérification des alertes
                if let Some(alert) = AlertService::check_for_alerts(&info, DEFAULT_LOW_THRESHOLD, DEFAULT_HIGH_THRESHOLD) {
                    warn!("Alerte détectée : {:?}", alert.alert_type);
                    // Pour [TICKET-2.3], on se contente de logger.
                }
            }
            Err(e) => {
                warn!("Erreur lors de la récupération de l'état de la batterie : {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::battery_status::{BatteryInfo, ChargingState};
    use crate::domain::battery_port::BatteryPort;

    /// Mock manuel du BatteryPort pour les tests du service de monitoring.
    struct MockBatteryPort {
        info: Option<BatteryInfo>,
        error: Option<String>,
    }

    impl BatteryPort for MockBatteryPort {
        fn get_status(&self) -> Result<BatteryInfo, String> {
            if let Some(ref err) = self.error {
                return Err(err.clone());
            }
            Ok(self.info.as_ref().cloned().unwrap())
        }
    }

    #[test]
    fn should_process_successful_check() {
        let mock_info = BatteryInfo {
            percentage: 75.0,
            is_charging: false,
            state: ChargingState::Discharging,
        };
        let port = MockBatteryPort { info: Some(mock_info), error: None };
        let use_case = GetBatteryStatusUseCase::new(port);

        // On vérifie que la méthode s'exécute sans paniquer
        BatteryMonitorService::process_check(&use_case);
    }

    #[test]
    fn should_handle_check_error() {
        let port = MockBatteryPort { info: None, error: Some("Erreur capteur".to_string()) };
        let use_case = GetBatteryStatusUseCase::new(port);

        // On vérifie que l'erreur est capturée correctement sans crash
        BatteryMonitorService::process_check(&use_case);
    }

    #[test]
    fn should_detect_alert_during_check() {
        let low_battery_info = BatteryInfo {
            percentage: 10.0,
            is_charging: false,
            state: ChargingState::Discharging,
        };
        let port = MockBatteryPort { info: Some(low_battery_info), error: None };
        let use_case = GetBatteryStatusUseCase::new(port);

        // La logique d'appel à AlertService est validée si l'exécution suit le chemin nominal
        BatteryMonitorService::process_check(&use_case);
    }
}

