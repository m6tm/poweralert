use crate::domain::config::AppConfig;

/// Port sortant pour l'accès aux configurations de l'application.
pub trait ConfigPort {
    /// Récupère la configuration actuelle.
    ///
    /// # Retourne
    /// Un `Result` contenant l'objet `AppConfig` ou un message d'erreur.
    fn load_config(&self) -> Result<AppConfig, String>;

    /// Sauvegarde la configuration fournie.
    ///
    /// # Arguments
    /// * `config` - Le nouvel objet `AppConfig` à persister.
    ///
    /// # Retourne
    /// Un `Result` vide ou un message d'erreur.
    fn save_config(&self, config: &AppConfig) -> Result<(), String>;
}
