use crate::domain::battery_status::BatteryInfo;
use crate::domain::battery_health::BatteryHealth;

/// Port définissant le contrat pour l'accès aux informations de batterie.
/// Implémenté par les adaptateurs d'infrastructure.
pub trait BatteryPort {
    /// Récupère les informations détaillées de la batterie.
    fn get_status(&self) -> Result<BatteryInfo, String>;
    
    /// Récupère les données de santé de la batterie (cycles, capacités).
    fn get_health(&self) -> Result<BatteryHealth, String>;
}
