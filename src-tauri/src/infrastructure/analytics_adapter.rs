use crate::domain::analytics_port::AnalyticsPort;
use crate::domain::battery_analytics::{BatterySnapshot, BatteryAnalytics};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

/// Chargé de la persistance des données analytiques dans un fichier JSON local.
pub struct AnalyticsAdapter {
    history_path: PathBuf,
}

impl AnalyticsAdapter {
    /// Crée un nouvel adaptateur. Utilise le dossier de configuration de l'application.
    pub fn new(app_handle: &tauri::AppHandle) -> Self {
        let history_dir = app_handle
            .path()
            .app_config_dir()
            .unwrap_or_else(|_| PathBuf::from("config"));
            
        let _ = fs::create_dir_all(&history_dir);
        let history_path = history_dir.join("history.json");
        
        Self { history_path }
    }
}

impl AnalyticsPort for AnalyticsAdapter {
    fn record_snapshot(&self, snapshot: BatterySnapshot) -> Result<(), String> {
        let mut history = self.load_history_internal();
        history.add_snapshot(snapshot);
        
        let content = serde_json::to_string_pretty(&history)
            .map_err(|e| format!("Erreur de sérialisation : {}", e))?;
            
        fs::write(&self.history_path, content)
            .map_err(|e| format!("Erreur d'écriture : {}", e))
    }

    fn load_history(&self) -> Result<Vec<BatterySnapshot>, String> {
        Ok(self.load_history_internal().snapshots)
    }

    fn clear_history(&self) -> Result<(), String> {
        if self.history_path.exists() {
            fs::remove_file(&self.history_path)
                .map_err(|e| format!("Erreur de suppression : {}", e))?;
        }
        Ok(())
    }
}

impl AnalyticsAdapter {
    fn load_history_internal(&self) -> BatteryAnalytics {
        if !self.history_path.exists() {
            return BatteryAnalytics::default();
        }

        let content = fs::read_to_string(&self.history_path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    }
}
