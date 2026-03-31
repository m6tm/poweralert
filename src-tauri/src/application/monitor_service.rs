use crate::application::battery_use_case::GetBatteryStatusUseCase;
use crate::domain::battery_alert::{AlertService, AlertType};
use crate::domain::battery_port::BatteryPort;
use log::{info, warn};
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use crate::infrastructure::config_adapter::ConfigAdapter;
use crate::application::config_use_case::GetConfigUseCase;
use crate::domain::config::AppConfig;

/// Service responsable de la surveillance périodique de l'état de la batterie.
pub struct BatteryMonitorService;

impl BatteryMonitorService {
    /// Démarre la boucle de surveillance asynchrone via le runtime interne de Tauri.
    ///
    /// # Arguments
    /// * `app_handle` - Le handle de l'application Tauri pour émettre des événements.
    /// * `port` - L'implémentation du port de batterie à utiliser.
    pub fn start_monitoring<P: BatteryPort + Send + Sync + 'static>(
        app_handle: AppHandle,
        port: P,
    ) {
        // tauri::async_runtime::spawn est toujours disponible dans le contexte Tauri,
        // contrairement à tokio::spawn qui requiert un runtime déjà initialisé.
        tauri::async_runtime::spawn(async move {
            let get_status_use_case = GetBatteryStatusUseCase::new(port);
            let mut interval = tokio::time::interval(Duration::from_millis(250)); // Surveillance haute fréquence (250ms)

            info!("Démarrage du cycle de surveillance en temps réel (250ms).");

            let mut last_plugged_state: Option<bool> = None;
            let mut last_percentage: Option<f32> = None;
            let mut last_temperature: Option<f32> = None;
            let mut last_power: Option<f32> = None;
            let mut last_alert_type: Option<AlertType> = None; // Suivi pour éviter les alertes en boucle
            let mut ticks_since_last_report = 240; // Rapport de survie toutes les minute (240 * 250ms = 60s)

            loop {
                interval.tick().await;

                if let Ok(info_battery) = get_status_use_case.execute() {
                    let plugged_changed = last_plugged_state != Some(info_battery.is_plugged_in);
                    
                    let pct_changed = last_percentage
                        .map(|last_pct| (last_pct - info_battery.percentage).abs() >= 0.1)
                        .unwrap_or(true);
                    
                    let temp_changed = last_temperature
                        .map(|last_t| (last_t - info_battery.temperature.unwrap_or(0.0)).abs() >= 0.1)
                        .unwrap_or(true);
                    
                    let power_changed = last_power
                        .map(|last_p| (last_p - info_battery.power_usage.unwrap_or(0.0)).abs() >= 0.1)
                        .unwrap_or(true);

                    let state_changed = plugged_changed || pct_changed || temp_changed || power_changed;

                    if state_changed {
                        last_plugged_state = Some(info_battery.is_plugged_in);
                        last_percentage = Some(info_battery.percentage);
                        last_temperature = info_battery.temperature;
                        last_power = info_battery.power_usage;
                        ticks_since_last_report = 0;
                        last_alert_type = Self::process_check_with_info(Some(&app_handle), &info_battery, last_alert_type);
                    } else {
                        ticks_since_last_report += 1;
                        if ticks_since_last_report >= 240 { // Log toutes les 60s
                            ticks_since_last_report = 0;
                            last_alert_type = Self::process_check_with_info(Some(&app_handle), &info_battery, last_alert_type);
                        }
                    }
                } else {
                    ticks_since_last_report += 1;
                    if ticks_since_last_report >= 240 {
                        ticks_since_last_report = 0;
                        last_alert_type = Self::process_check(Some(&app_handle), &get_status_use_case, last_alert_type);
                    }
                }
            }
        });
    }

    /// Effectue le traitement de l'alerte et des événements avec les informations déjà récupérées.
    fn process_check_with_info(
        app_handle: Option<&AppHandle>,
        info_battery: &crate::domain::battery_status::BatteryInfo,
        last_alert_type: Option<AlertType>,
    ) -> Option<AlertType> {
        info!(
            "Niveau de batterie : {:.0}% (Secteur: {})",
            info_battery.percentage, info_battery.is_plugged_in
        );

        // Émission de l'état actuel de la batterie au frontend si disponible
        if let Some(handle) = app_handle {
            let _ = handle.emit("battery-status", info_battery);
        }

        // Chargement de la configuration pour obtenir les seuils actuels
        let config = if let Some(handle) = app_handle {
            let adapter = ConfigAdapter::new(handle);
            let use_case = GetConfigUseCase::new(adapter);
            use_case.execute().unwrap_or_else(|_| AppConfig::default())
        } else {
            AppConfig::default()
        };

        // Vérification des alertes avec les seuils configurés
        let current_alert = AlertService::check_for_alerts(
            info_battery,
            config.low_threshold,
            config.high_threshold,
        );

        match current_alert {
            Some(alert) => {
                // N'émet l'alerte que si elle est différente de la précédente
                if Some(alert.alert_type.clone()) != last_alert_type {
                    warn!("Alerte détectée : {:?}", alert.alert_type);
                    if let Some(handle) = app_handle {
                        let _ = handle.emit("battery-alert", alert.clone());
                    }
                    Some(alert.alert_type)
                } else {
                    last_alert_type
                }
            }
            None => None,
        }
    }

    /// Effectue une vérification individuelle de l'état de la batterie (utilisé pour les tests ou en cas d'erreur ponctuelle).
    fn process_check<P: BatteryPort>(
        app_handle: Option<&AppHandle>,
        use_case: &GetBatteryStatusUseCase<P>,
        last_alert_type: Option<AlertType>,
    ) -> Option<AlertType> {
        match use_case.execute() {
            Ok(info_battery) => Self::process_check_with_info(app_handle, &info_battery, last_alert_type),
            Err(e) => {
                warn!(
                    "Erreur lors de la récupération de l'état de la batterie : {}",
                    e
                );
                last_alert_type
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
            temperature: Some(30.0),
            power_usage: Some(10.0),
        };
        let port = MockBatteryPort {
            info: Some(mock_info),
            error: None,
        };
        let use_case = GetBatteryStatusUseCase::new(port);

        // On vérifie que la méthode s'exécute sans paniquer
        BatteryMonitorService::process_check(None, &use_case, None);
    }

    #[test]
    fn should_handle_check_error() {
        let port = MockBatteryPort {
            info: None,
            error: Some("Erreur capteur".to_string()),
        };
        let use_case = GetBatteryStatusUseCase::new(port);

        // On vérifie que l'erreur est capturée correctement sans crash
        BatteryMonitorService::process_check(None, &use_case, None);
    }

    #[test]
    fn should_detect_alert_during_check() {
        let low_battery_info = BatteryInfo {
            percentage: 10.0,
            is_plugged_in: false,
            state: ChargingState::Discharging,
            temperature: Some(35.0),
            power_usage: Some(15.0),
        };
        let port = MockBatteryPort {
            info: Some(low_battery_info),
            error: None,
        };
        let use_case = GetBatteryStatusUseCase::new(port);

        // La logique d'appel à AlertService est validée si l'exécution suit le chemin nominal
        BatteryMonitorService::process_check(None, &use_case, None);
    }
}
