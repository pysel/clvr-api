use crate::clvr::model::clvr_model::CLVRModel;
use crate::trades::implementation::Trade;
use crate::trades::TradeDirection;
use alloy::primitives::U256;

#[cfg(test)]
mod tests {
    use crate::clvr::model::Omega;

    use super::*;

    const WEI: &str = "000000000000000000";

    fn size(x: u128) -> U256 {
        let x = x.to_string();
        let size: String = x.to_string() + WEI;
        U256::from_str_radix(&size, 10).unwrap()
    }

    #[test]
    fn test_clvr() {
        let test_cases: Vec<Omega> = vec![
            Omega::new_from(vec![
                Box::new(Trade::new(size(5), TradeDirection::Sell)),
                Box::new(Trade::new(size(10), TradeDirection::Buy)),
                Box::new(Trade::new(size(2), TradeDirection::Sell)),
            ]),
            Omega::new_from(vec![
                Box::new(Trade::new(size(5), TradeDirection::Sell)),
                Box::new(Trade::new(size(2), TradeDirection::Sell)),
                Box::new(Trade::new(size(10), TradeDirection::Buy)),
            ]),
            Omega::new_from(vec![
                Box::new(Trade::new(size(10), TradeDirection::Buy)),
                Box::new(Trade::new(size(5), TradeDirection::Sell)),
                Box::new(Trade::new(size(2), TradeDirection::Sell)),
            ]),
            Omega::new_from(vec![
                Box::new(Trade::new(size(10), TradeDirection::Buy)),
                Box::new(Trade::new(size(2), TradeDirection::Sell)),
                Box::new(Trade::new(size(5), TradeDirection::Sell)),
            ]),
        ];

        let expected: Omega = Omega::new_from(vec![
            Box::new(Trade::new(size(2), TradeDirection::Sell)),
            Box::new(Trade::new(size(5), TradeDirection::Sell)),
            Box::new(Trade::new(size(10), TradeDirection::Buy)),
        ]);

        let model = CLVRModel::new(size(100), size(100));

        let p_0 = U256::from(size(1));
        for mut test_case in test_cases {
            model.clvr_order(p_0, &mut test_case);
            assert!(test_case == expected);
        }
    }
}
