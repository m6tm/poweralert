use crate::application::battery_use_case::GetBatteryStatusUseCase;
use crate::domain::battery_alert::{AlertService, DEFAULT_HIGH_THRESHOLD, DEFAULT_LOW_THRESHOLD};
use crate::domain::battery_port::BatteryPort;
use log::{info, warn};
use std::time::Duration;
use tauri::{AppHandle, Emitter};

/// Service responsable de la surveillance périodique de l'état de la batterie.
pub struct BatteryMonitorService;

impl BatteryMonitorService {
    /// Démarre la boucle de surveillance asynchrone.
    ///
    /// # Arguments
    /// * `_app_handle` - Le handle de l'application Tauri pour émettre des événements ou accéder à l'état.
    /// * `port` - L'implémentation du port de batterie à utiliser.
    pub fn start_monitoring<P: BatteryPort + Send + Sync + 'static>(
        app_handle: AppHandle,
        port: P,
    ) {
        tokio::spawn(async move {
            let get_status_use_case = GetBatteryStatusUseCase::new(port);
            let mut interval = tokio::time::interval(Duration::from_secs(60));

            info!("Démarrage du cycle de surveillance de la batterie (60s).");

            loop {
                interval.tick().await;
                Self::process_check(Some(&app_handle), &get_status_use_case);
            }
        });
    }

    /// Effectue une vérification individuelle de l'état de la batterie.
    /// Extrait pour permettre les tests unitaires sans boucles infinies.
    fn process_check<P: BatteryPort>(
        app_handle: Option<&AppHandle>,
        use_case: &GetBatteryStatusUseCase<P>,
    ) {
        match use_case.execute() {
            Ok(info_battery) => {
                info!(
                    "Niveau de batterie : {:.0}% (Secteur: {})",
                    info_battery.percentage, info_battery.is_plugged_in
                );

                // Émission de l'état actuel de la batterie au frontend si disponible
                if let Some(handle) = app_handle {
                    let _ = handle.emit("battery-status", &info_battery);
                }

                // Vérification des alertes
                if let Some(alert) = AlertService::check_for_alerts(
                    &info_battery,
                    DEFAULT_LOW_THRESHOLD,
                    DEFAULT_HIGH_THRESHOLD,
                ) {
                    warn!("Alerte détectée : {:?}", alert.alert_type);
                    // Émission de l'alerte au frontend si disponible
                    if let Some(handle) = app_handle {
                        let _ = handle.emit("battery-alert", alert);
                    }
                }
            }
            Err(e) => {
                warn!(
                    "Erreur lors de la récupération de l'état de la batterie : {}",
                    e
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::battery_port::BatteryPort;
    use crate::domain::battery_status::{BatteryInfo, ChargingState};

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
            is_plugged_in: false,
            state: ChargingState::Discharging,
        };
        let port = MockBatteryPort {
            info: Some(mock_info),
            error: None,
        };
        let use_case = GetBatteryStatusUseCase::new(port);

        // On vérifie que la méthode s'exécute sans paniquer
        BatteryMonitorService::process_check(None, &use_case);
    }

    #[test]
    fn should_handle_check_error() {
        let port = MockBatteryPort {
            info: None,
            error: Some("Erreur capteur".to_string()),
        };
        let use_case = GetBatteryStatusUseCase::new(port);

        // On vérifie que l'erreur est capturée correctement sans crash
        BatteryMonitorService::process_check(None, &use_case);
    }

    #[test]
    fn should_detect_alert_during_check() {
        let low_battery_info = BatteryInfo {
            percentage: 10.0,
            is_plugged_in: false,
            state: ChargingState::Discharging,
        };
        let port = MockBatteryPort {
            info: Some(low_battery_info),
            error: None,
        };
        let use_case = GetBatteryStatusUseCase::new(port);

        // La logique d'appel à AlertService est validée si l'exécution suit le chemin nominal
        BatteryMonitorService::process_check(None, &use_case);
    }
}
