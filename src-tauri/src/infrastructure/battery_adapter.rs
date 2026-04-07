use battery::Manager;
use crate::domain::battery_status::{BatteryInfo, ChargingState};
use crate::domain::battery_health::BatteryHealth;
use crate::domain::battery_port::BatteryPort;

/// Adaptateur d'infrastructure permettant d'accéder aux informations de la batterie via le système.
/// Utilise la crate `battery` pour une compatibilité multiplateforme (dont Windows).
#[derive(Clone)]
pub struct BatteryAdapter;

impl BatteryAdapter {
    /// Initialise une nouvelle instance de l'adaptateur.
    pub fn new() -> Self {
        Self
    }

    /// Convertit les Joules (unité par défaut de la crate battery pour Energy) en Milliwatt-heures.
    fn joules_to_mwh(joules: f32) -> f32 {
        joules / 3.6
    }
}

impl BatteryPort for BatteryAdapter {
    /// Récupère l'état actuel de la batterie du système.
    fn get_status(&self) -> Result<BatteryInfo, String> {
        let manager = Manager::new().map_err(|e| format!("Erreur lors de l'initialisation du gestionnaire : {}", e))?;
        let mut batteries = manager.batteries().map_err(|e| format!("Impossible de lister les batteries : {}", e))?;
        
        if let Some(battery) = batteries.next() {
            let battery = battery.map_err(|e| format!("Erreur lors de la lecture des données de la batterie : {}", e))?;
            let percentage = (battery.state_of_charge().value * 100.0) as f32;
            
            let state = match battery.state() {
                battery::State::Charging => ChargingState::Charging,
                battery::State::Discharging => ChargingState::Discharging,
                battery::State::Full => ChargingState::Full,
                _ => ChargingState::Unknown,
            };

            let is_plugged_in = matches!(state, ChargingState::Charging | ChargingState::Full);
            let temperature = battery.temperature().map(|t| (t.value - 273.15) as f32);
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

    /// Récupère les données de santé physique de la batterie.
    fn get_health(&self) -> Result<BatteryHealth, String> {
        let manager = Manager::new().map_err(|e| format!("Erreur lors de l'initialisation du gestionnaire : {}", e))?;
        let mut batteries = manager.batteries().map_err(|e| format!("Impossible de lister les batteries : {}", e))?;
        
        if let Some(battery) = batteries.next() {
            let battery = battery.map_err(|e| format!("Erreur lors de la lecture des données de la batterie : {}", e))?;
            
            let cycle_count = battery.cycle_count();
            
            // La crate `battery` retourne l'énergie en Joules (SI) via le type Energy.
            // energy_full_design() et energy_full() ne sont pas des Option.
            let design_capacity = Some(Self::joules_to_mwh(battery.energy_full_design().value));
            let full_charge_capacity = Some(Self::joules_to_mwh(battery.energy_full().value));

            Ok(BatteryHealth::new(
                cycle_count,
                design_capacity,
                full_charge_capacity,
            ))
        } else {
            Err("Impossible de lire les données de santé : aucune batterie détectée".to_string())
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
