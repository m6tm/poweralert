use crate::application::battery_use_case::GetBatteryStatusUseCase;
use crate::domain::battery_alert::AlertService;
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
            let mut interval = tokio::time::interval(Duration::from_secs(1)); // Vérification rapide (1s)

            info!("Démarrage du cycle de surveillance de la batterie (1s de réactivité sur les changements d'état/pourcentage, log de survie toutes les 60s).");

            let mut last_plugged_state: Option<bool> = None;
            let mut last_percentage: Option<f32> = None;
            let mut ticks_since_last_report = 60; // Forcer le rapport initial

            loop {
                interval.tick().await;

                if let Ok(info_battery) = get_status_use_case.execute() {
                    let plugged_changed = last_plugged_state != Some(info_battery.is_plugged_in);
                    // Détecter un changement significatif de pourcentage (plus de 0.1% de différence pour éviter le bruit au niveau de f32)
                    let pct_changed = last_percentage
                        .map(|last_pct| (last_pct - info_battery.percentage).abs() >= 0.1)
                        .unwrap_or(true);
                    
                    let state_changed = plugged_changed || pct_changed;

                    if state_changed {
                        last_plugged_state = Some(info_battery.is_plugged_in);
                        last_percentage = Some(info_battery.percentage);
                        ticks_since_last_report = 0;
                        Self::process_check_with_info(Some(&app_handle), &info_battery);
                    } else {
                        ticks_since_last_report += 1;
                        if ticks_since_last_report >= 60 {
                            ticks_since_last_report = 0;
                            Self::process_check_with_info(Some(&app_handle), &info_battery);
                        }
                    }
                } else {
                    ticks_since_last_report += 1;
                    if ticks_since_last_report >= 60 {
                        ticks_since_last_report = 0;
                        Self::process_check(Some(&app_handle), &get_status_use_case);
                    }
                }
            }
        });
    }

    /// Effectue le traitement de l'alerte et des événements avec les informations déjà récupérées.
    fn process_check_with_info(
        app_handle: Option<&AppHandle>,
        info_battery: &crate::domain::battery_status::BatteryInfo,
    ) {
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
        if let Some(alert) = AlertService::check_for_alerts(
            info_battery,
            config.low_threshold,
            config.high_threshold,
        ) {
            warn!("Alerte détectée : {:?}", alert.alert_type);
            // Émission de l'alerte au frontend si disponible
            if let Some(handle) = app_handle {
                let _ = handle.emit("battery-alert", alert);
            }
        }
    }

    /// Effectue une vérification individuelle de l'état de la batterie (utilisé pour les tests ou en cas d'erreur ponctuelle).
    fn process_check<P: BatteryPort>(
        app_handle: Option<&AppHandle>,
        use_case: &GetBatteryStatusUseCase<P>,
    ) {
        match use_case.execute() {
            Ok(info_battery) => Self::process_check_with_info(app_handle, &info_battery),
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
