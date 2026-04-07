use serde::{Serialize, Deserialize};

/// Représente les données de santé physique de la batterie.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatteryHealth {
    /// Nombre de cycles de charge complets effectués par la batterie.
    pub cycle_count: Option<u32>,
    /// Capacité théorique d'usine de la batterie (en mWh).
    pub design_capacity: Option<f32>,
    /// Capacité maximale actuelle à pleine charge (en mWh).
    pub full_charge_capacity: Option<f32>,
    /// Niveau d'usure de la batterie (pourcentage de perte de capacité).
    /// Calculé comme : 1 - (full_charge_capacity / design_capacity).
    pub wear_level: Option<f32>,
}

impl BatteryHealth {
    /// Crée une nouvelle instance avec calcul automatique du niveau d'usure.
    pub fn new(
        cycle_count: Option<u32>,
        design_capacity: Option<f32>,
        full_charge_capacity: Option<f32>,
    ) -> Self {
        let wear_level = if let (Some(design), Some(full)) = (design_capacity, full_charge_capacity) {
            if design > 0.0 {
                // Le niveau d'usure est le pourcentage de capacité perdue
                Some((1.0 - (full / design)) * 100.0)
            } else {
                None
            }
        } else {
            None
        };

        Self {
            cycle_count,
            design_capacity,
            full_charge_capacity,
            wear_level,
        }
    }
}
