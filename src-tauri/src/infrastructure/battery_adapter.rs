use battery::Manager;
use crate::domain::battery_status::{BatteryInfo, ChargingState};
use crate::domain::battery_port::BatteryPort;

pub struct BatteryAdapter;

impl BatteryAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl BatteryPort for BatteryAdapter {
    fn get_status(&self) -> Result<BatteryInfo, String> {
        let manager = Manager::new().map_err(|e| format!("Erreur manager batterie: {}", e))?;
        
        let mut batteries = manager.batteries().map_err(|e| format!("Erreur liste batteries: {}", e))?;
        
        if let Some(battery) = batteries.next() {
            let battery = battery.map_err(|e| format!("Erreur lecture batterie: {}", e))?;
            
            let percentage = (battery.state_of_charge().value * 100.0) as f32;
            let state = match battery.state() {
                battery::State::Charging => ChargingState::Charging,
                battery::State::Discharging => ChargingState::Discharging,
                battery::State::Full => ChargingState::Full,
                _ => ChargingState::Unknown,
            };

            let is_charging = matches!(state, ChargingState::Charging);

            Ok(BatteryInfo {
                percentage,
                is_charging,
                state,
            })
        } else {
            Err("Aucune batterie détectée".to_string())
        }
    }
}
