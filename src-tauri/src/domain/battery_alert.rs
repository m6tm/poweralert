use crate::domain::battery_status::BatteryInfo;
use serde::{Serialize, Deserialize};

/// Types d'alertes disponibles pour la batterie.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AlertType {
    /// Alerte suggérant de brancher le chargeur (batterie faible).
    ConnectCharger,
    /// Alerte suggérant de débrancher le chargeur (batterie pleine).
    DisconnectCharger,
}

/// Structure représentant une alerte de batterie à destination de l'utilisateur.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatteryAlert {
    /// Type de l'alerte.
    pub alert_type: AlertType,
    /// Message détaillé à afficher à l'utilisateur.
    pub message: String,
}

/// Seuils par défaut configurés pour le système.
pub const DEFAULT_LOW_THRESHOLD: f32 = 50.0;
pub const DEFAULT_HIGH_THRESHOLD: f32 = 100.0;

/// Service de domaine responsable de la détection des conditions d'alerte.
pub struct AlertService;

impl AlertService {
    /// Vérifie si l'état actuel de la batterie nécessite le déclenchement d'une alerte avec les seuils par défaut.
    pub fn check_with_defaults(battery_info: &BatteryInfo) -> Option<BatteryAlert> {
        Self::check_for_alerts(battery_info, DEFAULT_LOW_THRESHOLD, DEFAULT_HIGH_THRESHOLD)
    }

    /// Vérifie si l'état actuel de la batterie nécessite le déclenchement d'une alerte.
    ///
    /// # Arguments
    /// * `battery_info` - Les informations actuelles de la batterie.
    /// * `low_threshold` - Le seuil de batterie faible configuré.
    /// * `high_threshold` - Le seuil de batterie pleine configuré.
    ///
    /// # Retourne
    /// Un `Option<BatteryAlert>` contenant l'alerte si nécessaire, sinon `None`.
    pub fn check_for_alerts(battery_info: &BatteryInfo, low_threshold: f32, high_threshold: f32) -> Option<BatteryAlert> {
        if !battery_info.is_plugged_in && battery_info.percentage <= low_threshold {
            return Some(BatteryAlert {
                alert_type: AlertType::ConnectCharger,
                message: format!("Batterie faible ({:.0}%). Branchez votre chargeur.", battery_info.percentage),
            });
        }
        
        if battery_info.is_plugged_in && battery_info.percentage >= high_threshold {
            return Some(BatteryAlert {
                alert_type: AlertType::DisconnectCharger,
                message: format!("Batterie pleine ({:.0}%). Débranchez votre chargeur.", battery_info.percentage),
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::battery_status::ChargingState;

    #[test]
    fn should_alert_to_connect_at_low_threshold() {
        let info = BatteryInfo {
            percentage: 50.0,
            is_plugged_in: false,
            state: ChargingState::Discharging,
        };
        let alert = AlertService::check_for_alerts(&info, 50.0, 100.0);
        assert!(alert.is_some());
        assert_eq!(alert.unwrap().alert_type, AlertType::ConnectCharger);
    }

    #[test]
    fn should_not_alert_if_already_charging_at_low_level() {
        let info = BatteryInfo {
            percentage: 40.0,
            is_plugged_in: true,
            state: ChargingState::Charging,
        };
        let alert = AlertService::check_for_alerts(&info, 50.0, 100.0);
        assert!(alert.is_none());
    }

    #[test]
    fn should_alert_to_disconnect_at_high_threshold() {
        let info = BatteryInfo {
            percentage: 100.0,
            is_plugged_in: true,
            state: ChargingState::Full,
        };
        let alert = AlertService::check_for_alerts(&info, 50.0, 100.0);
        assert!(alert.is_some());
        assert_eq!(alert.unwrap().alert_type, AlertType::DisconnectCharger);
    }

    #[test]
    fn should_work_with_defaults() {
        let info = BatteryInfo {
            percentage: 50.0,
            is_plugged_in: false,
            state: ChargingState::Discharging,
        };
        let alert = AlertService::check_with_defaults(&info);
        assert!(alert.is_some());
        assert_eq!(alert.unwrap().alert_type, AlertType::ConnectCharger);
    }

    #[test]
    fn should_not_alert_if_just_above_low_threshold() {
        let info = BatteryInfo {
            percentage: 50.1,
            is_plugged_in: false,
            state: ChargingState::Discharging,
        };
        let alert = AlertService::check_for_alerts(&info, 50.0, 100.0);
        assert!(alert.is_none());
    }

    #[test]
    fn should_not_alert_to_disconnect_if_below_high_threshold() {
        let info = BatteryInfo {
            percentage: 99.9,
            is_plugged_in: true,
            state: ChargingState::Charging,
        };
        let alert = AlertService::check_for_alerts(&info, 50.0, 100.0);
        assert!(alert.is_none());
    }

    #[test]
    fn should_not_alert_to_disconnect_at_high_threshold_if_discharging() {
        let info = BatteryInfo {
            percentage: 100.0,
            is_plugged_in: false,
            state: ChargingState::Discharging,
        };
        let alert = AlertService::check_for_alerts(&info, 50.0, 100.0);
        assert!(alert.is_none());
    }
}
