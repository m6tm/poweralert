use crate::domain::battery_status::BatteryInfo;

/// Port définissant le contrat pour l'accès aux informations de batterie.
/// Implémenté par les adaptateurs d'infrastructure.
pub trait BatteryPort {
    /// Récupère les informations détaillées de la batterie.
    fn get_status(&self) -> Result<BatteryInfo, String>;
}
