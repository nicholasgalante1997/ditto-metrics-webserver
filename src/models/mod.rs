pub mod models_mod {

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct JSONPingHealthCheck {
        pub token: String
    }

}