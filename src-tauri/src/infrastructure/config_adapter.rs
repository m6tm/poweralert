use crate::domain::config::AppConfig;
use crate::domain::config_port::ConfigPort;
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

/// Adaptateur d'infrastructure pour la gestion de la configuration via un fichier JSON.
pub struct ConfigAdapter {
    /// Chemin du fichier de configuration.
    config_path: PathBuf,
}

impl ConfigAdapter {
    /// Crée une nouvelle instance de l'adaptateur de configuration.
    /// Si le dossier parent n'existe pas, il sera créé lors de la première sauvegarde.
    pub fn new(app_handle: &tauri::AppHandle) -> Self {
        let config_dir = app_handle
            .path()
            .app_config_dir()
            .unwrap_or_else(|_| PathBuf::from("config"));
        
        let _ = fs::create_dir_all(&config_dir);
        let config_path = config_dir.join("config.json");
        
        Self { config_path }
    }
}

impl ConfigPort for ConfigAdapter {
    fn load_config(&self) -> Result<AppConfig, String> {
        if !self.config_path.exists() {
            return Ok(AppConfig::default());
        }

        let content = fs::read_to_string(&self.config_path)
            .map_err(|e| format!("Erreur lors de la lecture de la configuration : {}", e))?;

        serde_json::from_str(&content)
            .map_err(|e| format!("Erreur lors de la désérialisation de la configuration : {}", e))
    }

    fn save_config(&self, config: &AppConfig) -> Result<(), String> {
        let content = serde_json::to_string_pretty(config)
            .map_err(|e| format!("Erreur lors de la sérialisation de la configuration : {}", e))?;

        fs::write(&self.config_path, content)
            .map_err(|e| format!("Erreur lors de l'écriture de la configuration : {}", e))
    }
}
