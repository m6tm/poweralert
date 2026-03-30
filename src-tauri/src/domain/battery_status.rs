use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChargingState {
    Charging,
    Discharging,
    Full,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatteryInfo {
    pub percentage: f32,
    pub is_charging: bool,
    pub state: ChargingState,
}
