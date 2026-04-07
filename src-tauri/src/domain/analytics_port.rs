use crate::domain::battery_analytics::BatterySnapshot;

/// Port définissant les opérations de persistance pour les données analytiques.
pub trait AnalyticsPort {
    /// Enregistre une nouvelle capture dans l'historique.
    fn record_snapshot(&self, snapshot: BatterySnapshot) -> Result<(), String>;
    
    /// Récupère l'intégralité des captures historiques.
    fn load_history(&self) -> Result<Vec<BatterySnapshot>, String>;
    
    /// Efface l'historique (optionnel).
    fn clear_history(&self) -> Result<(), String>;
}
