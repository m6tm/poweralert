use battery::Manager;
use crate::domain::battery_status::{BatteryInfo, ChargingState};
use crate::domain::battery_port::BatteryPort;

/// Adaptateur d'infrastructure permettant d'accéder aux informations de la batterie via le système.
/// Utilise la crate `battery` pour une compatibilité multiplateforme (dont Windows).
pub struct BatteryAdapter;

impl BatteryAdapter {
    /// Initialise une nouvelle instance de l'adaptateur.
    pub fn new() -> Self {
        Self
    }
}

impl BatteryPort for BatteryAdapter {
    /// Récupère l'état actuel de la batterie du système.
    ///
    /// # Retourne
    /// - `Ok(BatteryInfo)` : Si les informations ont pu être récupérées avec succès.
    /// - `Err(String)` : Un message d'erreur si l'accès à la batterie échoue.
    fn get_status(&self) -> Result<BatteryInfo, String> {
        // Initialisation du gestionnaire de batterie.
        // Sur Windows, le gestionnaire n'est pas thread-safe (Rc), il doit être instancié à chaque appel.
        let manager = Manager::new().map_err(|e| format!("Erreur lors de l'initialisation du gestionnaire : {}", e))?;
        
        // Récupération de la liste des batteries disponibles
        let mut batteries = manager.batteries().map_err(|e| format!("Impossible de lister les batteries : {}", e))?;
        
        // On récupère la première batterie disponible
        if let Some(battery) = batteries.next() {
            let battery = battery.map_err(|e| format!("Erreur lors de la lecture des données de la batterie : {}", e))?;
            
            // Calcul du pourcentage de charge actuel
            let percentage = (battery.state_of_charge().value * 100.0) as f32;
            
            // Mapping de l'état de charge vers l'énumération du domaine
            let state = match battery.state() {
                battery::State::Charging => ChargingState::Charging,
                battery::State::Discharging => ChargingState::Discharging,
                battery::State::Full => ChargingState::Full,
                _ => ChargingState::Unknown,
            };

            // La batterie est considérée comme "branchée" si elle charge ou si elle est pleine.
            let is_plugged_in = matches!(state, ChargingState::Charging | ChargingState::Full);

            // Récupération de la température (Kelvin -> Celsius)
            let temperature = battery.temperature()
                .map(|t| (t.value - 273.15) as f32);
            
            // Récupération de la puissance (Watts)
            let power_usage = Some(battery.energy_rate().value as f32);

            Ok(BatteryInfo {
                percentage,
                is_plugged_in,
                state,
                temperature,
                power_usage,
            })
        } else {
            Err("Aucune batterie n'a été détectée par le système".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tente de contacter l'API de batterie du système hôte.
    /// Valide soit la récupération des données, soit un message d'erreur cohérent.
    fn should_interact_with_system_battery() {
        let adapter = BatteryAdapter::new();
        let result = adapter.get_status();
        
        match result {
            Ok(info) => {
                // Si une batterie est présente, ses valeurs doivent être plausibles.
                assert!(info.percentage >= 0.0 && info.percentage <= 100.0);
                println!("Batterie système détectée : {}%", info.percentage);
            },
            Err(e) => {
                // Si pas de batterie (ex: PC fixe), on vérifie que le message est géré.
                assert!(e.contains("détectée") || e.contains("Erreur"));
            }
        }
    }
}
