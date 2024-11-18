use crate::clvr::model::clvr_model::CLVRModel;
use crate::clvr::model::{Model, Omega};
use alloy::primitives::U256;
use rug::ops::Pow;
use rug::{Float, Integer};

// const LOG_E_WEI_STR: &str = "42446531673892822312";
// const WEI: &str = "000000000000000000";
// const LOG_E_WEI: Lazy<U256> = Lazy::new(|| U256::from_str_radix(LOG_E_WEI_STR, 10).expect("Failed to convert string to U256"));
// const BASE: Lazy<U256> = Lazy::new(|| U256::from_str_radix("10000000000000000000", 10).unwrap());

// NOTE: actually computes log_{e * 10 ** 18}{x} so that the result is accurate for a wei representation of a number
// A property used is log_a(x) = log_b(x) / log_b(a)
fn ln(x: U256) -> Float {
    let x_int =
        Integer::from_str_radix(&x.to_string(), 10).expect("Failed to convert U256 to Integer");
    let x_float: Float = Float::with_val(256, &x_int) * Float::with_val(18, &10).pow(-18);
    let ln_x = x_float.ln();

    ln_x
}

impl CLVRModel {
    pub fn clvr_order(&self, p_0: U256, omega: &mut Omega) {
        let size = omega.len();
        let ln_p0 = ln(p_0);
        let two = Float::with_val(18, &2);

        // think of this as a selection sort algorithm
        // iterating through 1 to size+1 because omega is 1-indexed
        for t in 1..size + 1 {
            // select t'th trade by minimizing ( ln(p_0) - ln(P(o, t)) )^2
            let mut candidate_index = t;
            let mut candidate_value = (ln_p0.clone() - ln(self.P(omega, t))).pow(two.clone());

            for i in t + 1..size + 1 {
                // try each trade at position t
                omega.swap(t, i); // simulate that trade i is at position t

                let value = (ln_p0.clone() - ln(self.P(omega, t))).pow(two.clone()); // compute the value for this omega

                if value < candidate_value {
                    candidate_index = i;
                    candidate_value = value;
                }

                omega.swap(i, t); // swap back to preserve original state
            }

            if t != candidate_index {
                // if omega exists with a better value, swap to that omega
                omega.swap(candidate_index, t);
            }
        }
    }
}
