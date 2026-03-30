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

/// Service de domaine responsable de la détection des conditions d'alerte.
pub struct AlertService;

impl AlertService {
    /// Vérifie si l'état actuel de la batterie nécessite le déclenchement d'une alerte.
    ///
    /// # Arguments
    /// * `percentage` - Le pourcentage actuel de la batterie.
    /// * `is_charging` - L'état de charge actuel.
    /// * `low_threshold` - Le seuil de batterie faible configuré.
    /// * `high_threshold` - Le seuil de batterie pleine configuré.
    ///
    /// # Retourne
    /// Un `Option<BatteryAlert>` contenant l'alerte si nécessaire, sinon `None`.
    pub fn check_for_alerts(percentage: f32, is_charging: bool, low_threshold: f32, high_threshold: f32) -> Option<BatteryAlert> {
        if !is_charging && percentage <= low_threshold {
            return Some(BatteryAlert {
                alert_type: AlertType::ConnectCharger,
                message: format!("Batterie faible ({}%). Branchez votre chargeur.", percentage as i32),
            });
        }
        
        if is_charging && percentage >= high_threshold {
            return Some(BatteryAlert {
                alert_type: AlertType::DisconnectCharger,
                message: format!("Batterie pleine ({}%). Débranchez votre chargeur.", percentage as i32),
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_alert_to_connect_at_low_threshold() {
        let alert = AlertService::check_for_alerts(50.0, false, 50.0, 100.0);
        assert!(alert.is_some());
        assert_eq!(alert.unwrap().alert_type, AlertType::ConnectCharger);
    }

    #[test]
    fn should_not_alert_if_already_charging_at_low_level() {
        let alert = AlertService::check_for_alerts(40.0, true, 50.0, 100.0);
        assert!(alert.is_none());
    }

    #[test]
    fn should_alert_to_disconnect_at_high_threshold() {
        let alert = AlertService::check_for_alerts(100.0, true, 50.0, 100.0);
        assert!(alert.is_some());
        assert_eq!(alert.unwrap().alert_type, AlertType::DisconnectCharger);
    }
}
