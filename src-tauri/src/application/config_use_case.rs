use crate::domain::config::AppConfig;
use crate::domain::config_port::ConfigPort;

/// Cas d'utilisation pour récupérer la configuration.
pub struct GetConfigUseCase<P: ConfigPort> {
    port: P,
}

impl<P: ConfigPort> GetConfigUseCase<P> {
    pub fn new(port: P) -> Self {
        Self { port }
    }

    pub fn execute(&self) -> Result<AppConfig, String> {
        self.port.load_config()
    }
}

/// Cas d'utilisation pour sauvegarder la configuration.
pub struct SaveConfigUseCase<P: ConfigPort> {
    port: P,
}

impl<P: ConfigPort> SaveConfigUseCase<P> {
    pub fn new(port: P) -> Self {
        Self { port }
    }

    pub fn execute(&self, config: AppConfig) -> Result<(), String> {
        self.port.save_config(&config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock du ConfigPort simulant un stockage en mémoire.
    struct MockConfigPort {
        stored: std::cell::RefCell<Option<AppConfig>>,
        should_fail: bool,
    }

    impl MockConfigPort {
        fn success(initial: Option<AppConfig>) -> Self {
            Self { stored: std::cell::RefCell::new(initial), should_fail: false }
        }

        fn failing() -> Self {
            Self { stored: std::cell::RefCell::new(None), should_fail: true }
        }
    }

    impl ConfigPort for MockConfigPort {
        fn load_config(&self) -> Result<AppConfig, String> {
            if self.should_fail {
                return Err("Erreur de lecture simulée".to_string());
            }
            Ok(self.stored.borrow().clone().unwrap_or_default())
        }

        fn save_config(&self, config: &AppConfig) -> Result<(), String> {
            if self.should_fail {
                return Err("Erreur d'écriture simulée".to_string());
            }
            *self.stored.borrow_mut() = Some(config.clone());
            Ok(())
        }
    }

    #[test]
    fn should_return_default_config_when_none_stored() {
        let port = MockConfigPort::success(None);
        let use_case = GetConfigUseCase::new(port);

        let result = use_case.execute();

        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.low_threshold, 50.0);
        assert_eq!(config.high_threshold, 100.0);
    }

    #[test]
    fn should_return_stored_config() {
        let stored = AppConfig { low_threshold: 20.0, high_threshold: 80.0, run_at_startup: false };
        let port = MockConfigPort::success(Some(stored));
        let use_case = GetConfigUseCase::new(port);

        let result = use_case.execute();

        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.low_threshold, 20.0);
        assert_eq!(config.high_threshold, 80.0);
        assert!(!config.run_at_startup);
    }

    #[test]
    fn should_forward_load_error() {
        let port = MockConfigPort::failing();
        let use_case = GetConfigUseCase::new(port);

        let result = use_case.execute();

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Erreur de lecture simulée");
    }

    #[test]
    fn should_persist_config_via_port() {
        let port = MockConfigPort::success(None);
        let new_config = AppConfig { low_threshold: 30.0, high_threshold: 90.0, run_at_startup: true };
        let use_case = SaveConfigUseCase::new(port);

        let result = use_case.execute(new_config);

        assert!(result.is_ok());
    }

    #[test]
    fn should_forward_save_error() {
        let port = MockConfigPort::failing();
        let use_case = SaveConfigUseCase::new(port);

        let result = use_case.execute(AppConfig::default());

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Erreur d'écriture simulée");
    }
}
