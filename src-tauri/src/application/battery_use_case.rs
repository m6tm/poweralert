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
