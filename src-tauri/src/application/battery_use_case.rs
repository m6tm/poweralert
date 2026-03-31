use crate::domain::battery_status::BatteryInfo;
use crate::domain::battery_port::BatteryPort;

/// Cas d'utilisation permettant de récupérer l'état de la batterie.
/// Abstraction agnostique de l'infrastructure via le port BatteryPort.
pub struct GetBatteryStatusUseCase<P: BatteryPort> {
    port: P,
}

impl<P: BatteryPort> GetBatteryStatusUseCase<P> {
    /// Crée une nouvelle instance du cas d'utilisation avec un port donné.
    pub fn new(port: P) -> Self {
        Self { port }
    }

    /// Exécute la logique de récupération de l'état de la batterie.
    pub fn execute(&self) -> Result<BatteryInfo, String> {
        self.port.get_status()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::battery_status::ChargingState;

    /// Mock manuel du BatteryPort pour isoler le test.
    struct MockBatteryPort {
        should_fail: bool,
    }

    impl BatteryPort for MockBatteryPort {
        fn get_status(&self) -> Result<BatteryInfo, String> {
            if self.should_fail {
                return Err("Erreur simulée".to_string());
            }
            Ok(BatteryInfo {
                percentage: 85.0,
                is_plugged_in: true,
                state: ChargingState::Charging,
                temperature: None,
                power_usage: None,
            })
        }
    }

    #[test]
    fn should_return_battery_info_from_port() {
        let port = MockBatteryPort { should_fail: false };
        let use_case = GetBatteryStatusUseCase::new(port);
        
        let result = use_case.execute();
        
        assert!(result.is_ok());
        let info = result.unwrap();
        assert_eq!(info.percentage, 85.0);
        assert!(info.is_plugged_in);
        assert!(matches!(info.state, ChargingState::Charging));
    }

    #[test]
    fn should_forward_error_from_port() {
        let port = MockBatteryPort { should_fail: true };
        let use_case = GetBatteryStatusUseCase::new(port);
        
        let result = use_case.execute();
        
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Erreur simulée");
    }
}
