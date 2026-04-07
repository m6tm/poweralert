use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Représente un point de données historique de la batterie.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatterySnapshot {
    /// Horodatage Unix de la mesure.
    pub timestamp: u64,
    /// Pourcentage de charge au moment de la mesure.
    pub percentage: f32,
    /// Autonomie estimée en minutes.
    pub autonomy_minutes: Option<u32>,
    /// Niveau d'usure de la batterie (pourcentage).
    pub wear_level: Option<f32>,
}

impl BatterySnapshot {
    /// Crée une nouvelle capture avec l'horodatage actuel.
    pub fn now(percentage: f32, autonomy_minutes: Option<u32>, wear_level: Option<f32>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
            
        Self {
            timestamp,
            percentage,
            autonomy_minutes,
            wear_level,
        }
    }
}

/// Conteneur pour l'historique complet des mesures.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatteryAnalytics {
    /// Liste chronologique des captures.
    pub snapshots: Vec<BatterySnapshot>,
}

impl BatteryAnalytics {
    /// Ajoute une nouvelle capture et limite la taille de l'historique (ex: 1000 derniers points).
    pub fn add_snapshot(&mut self, snapshot: BatterySnapshot) {
        self.snapshots.push(snapshot);
        // On garde par exemple les 1000 derniers points pour éviter une croissance infinie.
        if self.snapshots.len() > 1000 {
            self.snapshots.remove(0);
        }
    }
}
