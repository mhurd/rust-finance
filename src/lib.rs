mod financial {

    fn cash_flow_present_value_discrete(
        cash_flow_times: &Vec<usize>,
        cash_flow_amounts: &Vec<f64>,
        market_rate: f64,
    ) -> f64 {
        let mut present_value = 0.0;
        let cash_flows = cash_flow_times.iter().zip(cash_flow_amounts.iter());
        for (time, cash) in cash_flows {
            present_value += cash / num::pow(1.0 + market_rate, *time);
        }
        present_value
    }

    fn cash_flow_present_value_continuous(
        cash_flow_times: &Vec<usize>,
        cash_flow_amounts: &Vec<f64>,
        market_rate: f64,
    ) -> f64 {
        let mut present_value = 0.0;
        let cash_flows = cash_flow_times.iter().zip(cash_flow_amounts.iter());
        for (time, cash) in cash_flows {
            present_value += cash / (*time as f64 * market_rate).exp();
        }
        present_value
    }

    fn fill<T: Copy>(size: usize, item: T) -> Vec<T> {
        let mut fill_vec: Vec<T> = Vec::with_capacity(size as usize);
        for _i in 0..size {
            fill_vec.push(item);
        }
        return fill_vec;
    }

    fn bond_price_discrete(
        cash_flow_times: &Vec<usize>,
        coupon: f64,
        principal: f64,
        market_rate: f64,
    ) -> f64 {
        let num_of_cash_flows = cash_flow_times.len();
        let coupon_payments: Vec<f64> = fill(num_of_cash_flows, principal * coupon);
        let mut present_value =
            cash_flow_present_value_discrete(cash_flow_times, &coupon_payments, market_rate);
        // now include the present value of the final period repayment of the principal
        let principal_repayment_time = cash_flow_times.last().expect("Cash flow times is empty!");
        present_value += principal / num::pow(1.0 + market_rate, *principal_repayment_time);
        present_value
    }

    fn bond_price_continuous(
        cash_flow_times: &Vec<usize>,
        coupon: f64,
        principal: f64,
        market_rate: f64,
    ) -> f64 {
        let num_of_cash_flows = cash_flow_times.len();
        let coupon_payments: Vec<f64> = fill(num_of_cash_flows, principal * coupon);
        let mut present_value =
            cash_flow_present_value_continuous(cash_flow_times, &coupon_payments, market_rate);
        // now include the present value of the final period repayment of the principal
        let principal_repayment_time = cash_flow_times.last().expect("Cash flow times is empty!");
        present_value += principal / (*principal_repayment_time as f64 * market_rate).exp();
        present_value
    }

    #[cfg(test)]
    mod tests {

        #[test]
        fn check_cash_flow_present_value_discrete() {
            // 1M in a year from now at 1.2% APR
            let times = vec![1];
            let cash = vec![1_000_000.0];
            let market_rate = 0.012;
            let present_value = super::cash_flow_present_value_discrete(&times, &cash, market_rate);
            assert_eq!(
                988142.2924901185, present_value,
                "Testing present Value of £1M in a year at 1.2% = {} = £{:.2}",
                present_value, present_value
            );

            // 1M five years from now at 1.5% APR
            let times = vec![5];
            let cash = vec![1_000_000.0];
            let market_rate = 0.015;
            let present_value = super::cash_flow_present_value_discrete(&times, &cash, market_rate);
            assert_eq!(
                928260.32540564, present_value,
                "Testing present Value of £1M in 5 years at 1.5% = {} = £{:.2}",
                present_value, present_value
            );

            // 500K a year for five years at rates of 1.5%
            let times = vec![1, 2, 3, 4, 5];
            let cash = vec![500_000.0, 500_000.0, 500_000.0, 500_000.0, 500_000.0];
            let market_rate = 0.015;
            let present_value = super::cash_flow_present_value_discrete(&times, &cash, market_rate);
            assert_eq!(
                2391322.4864786887, present_value,
                "testing present Value of £500K a year for 5 years at 1.5% = {} = £{:.2}",
                present_value, present_value
            );
        }

        #[test]
        fn check_cash_flow_present_value_continuous() {
            // 1M in a year from now at 1.5% APR continuously compounded
            let times = vec![1];
            let cash = vec![1_000_000.0];
            let market_rate = 0.015;
            let present_value = super::cash_flow_present_value_continuous(&times, &cash, market_rate);
            assert_eq!(985111.9396030627, present_value,
                       "Testing present Value of £1M in a year at 1.5% continuously compounded = {} = £{:.2}",
                       present_value, present_value);

            // 1M five years from now at 1.5% APR
            let times = vec![5];
            let cash = vec![1_000_000.0];
            let market_rate = 0.015;
            let present_value = super::cash_flow_present_value_continuous(&times, &cash, market_rate);
            assert_eq!(927743.486328553, present_value,
                       "Testing present Value of £1M in 5 years at 1.5% continuously compounded = {} = £{:.2}",
                       present_value, present_value);

            // 500K a year for five years at rates of 1.5%
            let times = vec![1, 2, 3, 4, 5];
            let cash = vec![500_000.0, 500_000.0, 500_000.0, 500_000.0, 500_000.0];
            let market_rate = 0.015;
            let present_value = super::cash_flow_present_value_continuous(&times, &cash, market_rate);
            assert_eq!(2390531.487448736, present_value,
                       "Testing present Value of £500K a year for 5 years at 1.5% continuously compounded = {} = £{:.2}",
                       present_value, present_value);
        }

        #[test]
        fn check_bond_price_discrete() {
            // 3-year $100 bond with a 1.1% APR semi-annual coupon compounded discretely
            let times = vec![1, 2, 3, 4, 5, 6];
            let coupon = 0.0055; // 0.011 / 2 as this is semi-annual
            let principal: f64 = 100.0;
            let market_rate = 0.0075; // 1.5% APR for 6-months
            let present_value = super::bond_price_discrete(&times, coupon, principal, market_rate);
            assert_eq!(98.83088047394604, present_value,
                       "3-year $100 bond with a 1.1% APR semi-annual coupon compounded discretely = {} = £{:.2}",
                       present_value, present_value);
        }

        #[test]
        fn check_bond_price_continuous() {
            // 3-year $100 bond with a 1.1% APR semi-annual coupon compounded continuously
            let times = vec![1, 2, 3, 4, 5, 6];
            let coupon = 0.0055; // 0.011 / 2 as this is semi-annual
            let principal: f64 = 100.0;
            let market_rate = 0.0075; // 1.5% APR for 6-months
            let present_value = super::bond_price_continuous(&times, coupon, principal, market_rate);
            assert_eq!(98.81451394890486, present_value,
                       "3-year $100 bond with a 1.1% APR semi-annual coupon compounded continuously = {} = £{:.2}",
                       present_value, present_value);
        }

    }

}
