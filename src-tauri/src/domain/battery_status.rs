use serde::{Serialize, Deserialize};

/// Représente les différents états de charge possibles pour une batterie.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChargingState {
    /// La batterie est en train de se charger.
    Charging,
    /// La batterie se décharge (utilisation normale sur batterie).
    Discharging,
    /// La batterie est complètement chargée.
    Full,
    /// L'état de la batterie ne peut pas être déterminé.
    Unknown,
}

/// Contient les informations détaillées sur l'état d'une batterie.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatteryInfo {
    /// Pourcentage de charge actuel (de 0.0 à 100.0).
    pub percentage: f32,
    /// Indique si la batterie est actuellement alimentée par une source externe (secteur).
    pub is_plugged_in: bool,
    /// État précis de la charge (Chargement, Déchargement, Pleine, Inconnu).
    pub state: ChargingState,
    /// Température actuelle de la batterie en degrés Celsius (si disponible).
    pub temperature: Option<f32>,
    /// Puissance instantanée consommée ou reçue en Watts (si disponible).
    pub power_usage: Option<f32>,
}
