pub mod models_factory {

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct WebServiceHealthCheck {
        pub token: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct AvailabilityMetric {
        pub m_type: String,
        pub value: AvailabilityMetricValue,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct AvailabilityMetricValue {
        pub fn_name: String,
        pub availability: u8,
        pub timestamp: String,
    }

    impl AvailabilityMetric {
        pub fn build_metric_string(&self) -> String {
            format!(
                "metric_type={},fn_name={},availability={},timestamp={}",
                self.m_type, self.value.fn_name, self.value.availability, self.value.timestamp
            )
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TimingMetricValue {
        pub fn_name: String,
        pub duration: usize,
        pub timestamp: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TimingMetric {
        pub m_type: String,
        pub value: TimingMetricValue
    }

    impl TimingMetric {
        pub fn build_metric_string(&self) -> String {
            format!(
                "metric_type={},fn_name={},duration={},timestamp={}",
                self.m_type, self.value.fn_name, self.value.duration, self.value.timestamp
            )
        }
    }
}
