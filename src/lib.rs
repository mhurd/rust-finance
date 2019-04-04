mod financial {

    fn cash_flow_present_value_discrete(
        cash_flow_times: &[usize],
        cash_flow_amounts: &[f64],
        rate: f64,
    ) -> f64 {
        let mut present_value = 0.0;
        let cash_flows = cash_flow_times.iter().zip(cash_flow_amounts.iter());
        for (time, cash) in cash_flows {
            present_value += cash / num::pow(1.0 + rate, *time);
        }
        present_value
    }

    fn cash_flow_present_value_continuous(
        cash_flow_times: &[usize],
        cash_flow_amounts: &[f64],
        rate: f64,
    ) -> f64 {
        let mut present_value = 0.0;
        let cash_flows = cash_flow_times.iter().zip(cash_flow_amounts.iter());
        for (time, cash) in cash_flows {
            present_value += cash / (*time as f64 * rate).exp();
        }
        present_value
    }

    #[cfg(test)]
    mod tests {

        #[test]
        fn check_cash_flow_present_value_discrete() {

            // 1M in a year from now at 1.2% APR
            let times = [1];
            let cash = [1_000_000.0];
            let present_value = super::cash_flow_present_value_discrete(&times, &cash, 0.012);
            assert_eq!(988142.2924901185, present_value,
                       "Testing present Value of £1M in a year at 1.2% = {} = £{:.2}",
                       present_value, present_value);

            // 1M five years from now at 1.5% APR
            let times = [5];
            let cash = [1_000_000.0];
            let present_value = super::cash_flow_present_value_discrete(&times, &cash, 0.015);
            assert_eq!(928260.32540564, present_value,
                       "Testing present Value of £1M in 5 years at 1.5% = {} = £{:.2}",
                       present_value, present_value);

            // 500K a year for five years at rates of 1.5%
            let times = [1, 2, 3, 4, 5];
            let cash = [500_000.0, 500_000.0, 500_000.0, 500_000.0, 500_000.0];
            let present_value = super::cash_flow_present_value_discrete(&times, &cash, 0.015);
            assert_eq!(2391322.4864786887, present_value,
                       "testing present Value of £500K a year for 5 years at 1.5% = {} = £{:.2}",
                       present_value, present_value);
        }

        #[test]
        fn check_cash_flow_present_value_continuous() {

            // 1M in a year from now at 1.5% APR continuously compounded
            let times = [1];
            let cash = [1_000_000.0];
            let present_value = super::cash_flow_present_value_continuous(&times, &cash, 0.015);
            assert_eq!(985111.9396030627, present_value,
                       "Testing present Value of £1M in a year at 1.5% continuously compounded = {} = £{:.2}",
                       present_value, present_value);

            // 1M five years from now at 1.5% APR
            let times = [5];
            let cash = [1_000_000.0];
            let present_value = super::cash_flow_present_value_continuous(&times, &cash, 0.015);
            assert_eq!(927743.486328553, present_value,
                       "Testing present Value of £1M in 5 years at 1.5% continuously compounded = {} = £{:.2}",
                       present_value, present_value);

            // 500K a year for five years at rates of 1.5%
            let times = [1, 2, 3, 4, 5];
            let cash = [500_000.0, 500_000.0, 500_000.0, 500_000.0, 500_000.0];
            let present_value = super::cash_flow_present_value_continuous(&times, &cash, 0.015);
            assert_eq!(2390531.487448736, present_value,
                       "Testing present Value of £500K a year for 5 years at 1.5% continuously compounded = {} = £{:.2}",
                       present_value, present_value);
        }

    }

}
