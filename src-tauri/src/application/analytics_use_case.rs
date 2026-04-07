use crate::domain::analytics_port::AnalyticsPort;
use crate::domain::battery_analytics::BatterySnapshot;

/// Cas d'utilisation pour récupérer l'historique analytique.
pub struct GetAnalyticsUseCase<P: AnalyticsPort> {
    port: P,
}

impl<P: AnalyticsPort> GetAnalyticsUseCase<P> {
    pub fn new(port: P) -> Self {
        Self { port }
    }

    pub fn execute(&self) -> Result<Vec<BatterySnapshot>, String> {
        self.port.load_history()
    }
}

/// Cas d'utilisation pour enregistrer une capture.
pub struct RecordSnapshotUseCase<P: AnalyticsPort> {
    port: P,
}

impl<P: AnalyticsPort> RecordSnapshotUseCase<P> {
    pub fn new(port: P) -> Self {
        Self { port }
    }

    pub fn execute(&self, snapshot: BatterySnapshot) -> Result<(), String> {
        self.port.record_snapshot(snapshot)
    }
}
