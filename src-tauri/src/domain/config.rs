use serde::{Serialize, Deserialize};

/// Structure représentant la configuration de l'application.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    /// Seuil de batterie faible pour l'alerte (%)
    pub low_threshold: f32,
    /// Seuil de batterie pleine pour l'alerte (%)
    pub high_threshold: f32,
    /// Indicateur si l'application doit se lancer au démarrage
    pub run_at_startup: bool,
    /// Indicateur si la fenêtre doit rester minimisée au démarrage
    pub start_minimized: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            low_threshold: 50.0,
            high_threshold: 100.0,
            run_at_startup: true,
            start_minimized: false,
        }
    }
}
