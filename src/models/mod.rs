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
        pub service_name: String,
        pub fn_name: String,
        pub availability: u8,
        pub timestamp: String,
    }

    impl AvailabilityMetric {
        pub fn build_metric_string(&self) -> String {
            format!(
                "***************\nmetric_type={}\nservice_name={}\nfn_name={}\navailability={}\ntimestamp={}\n***************\n",
                self.m_type, self.value.service_name, self.value.fn_name, self.value.availability, self.value.timestamp
            )
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TimingMetricValue {
        pub fn_name: String,
        pub service_name: String,
        pub duration: String,
        pub timestamp: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TimingMetric {
        pub m_type: String,
        pub value: TimingMetricValue,
    }

    impl TimingMetric {
        pub fn build_metric_string(&self) -> String {
            format!(
                "***************\nmetric_type={}\nservice_name={}\nfn_name={}\nduration={}\ntimestamp={}\n***************\n",
                self.m_type, self.value.service_name, self.value.fn_name, self.value.duration, self.value.timestamp
            )
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CountMetric {
        pub m_type: String,
        pub value: CountMetricValue,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CountMetricValue {
        service_name: String,
        fn_name: String,
        count_adjustment: u8,
        s: String,
        timestamp: String,
    }

    impl CountMetric {
        pub fn build_metric_string(&self) -> String {
            format!(
                "***************\nmetric_type={}\nservice_name={}\nfn_name={}\nmetadata={}\ncount_adjustment={}\ntimestamp={}\n***************\n",
                self.m_type, self.value.service_name, self.value.fn_name, self.value.s, self.value.count_adjustment, self.value.timestamp
            )
        }
    }
}
