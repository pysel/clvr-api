use alloy::primitives::U256;

use crate::clvr::model::{clvr_model::CLVRModel, Omega};
use crate::trades::ITrade;

// CoreScheduler is responsible for performing one instance of the algorithm. Called by executor. (TODO: probably rename these two)
pub struct CoreScheduler {
    omega: Omega,
    model: CLVRModel,
}

impl CoreScheduler {
    pub fn new(reserve_y: U256, reserve_x: U256) -> Self {
        // create variables related to the algorithm
        let omega = Omega::new();
        let model = CLVRModel::new(reserve_y, reserve_x);

        Self { omega, model }
    }

    fn add_trade(&mut self, trade: Box<dyn ITrade>) {
        self.omega.push(trade);
    }
}
