pub mod postgres_client {

    use serde::{Serialize,Deserialize};
    use postgres::{Client, NoTls, Error};
    use std::env;

    // https://docs.rs/postgres/latest/postgres/config/struct.Config.html
    // default port is 5432

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PostgresClientService {
        pub database: String,
        pub user: String,
        pub password: String,
        pub host: String,
        pub port: u16
    }

    impl PostgresClientService {
        pub fn connect(&self) -> Result<Client, Error> {
            Client::connect(format!("host={} user={} dbname={} host={} port={}", self.host, self.user, self.database, self.host, self.port).as_str(), NoTls)
        }

        pub fn get_postgres_client_instance() -> PostgresClientService {
            PostgresClientService {
                database: env::var("POSTGRES_DATABASE").expect("Environment Variable 'POSTGRES_DATABASE' is unset. Check to see environment variables are being set/loaded correctly."),
                host: env::var("POSTGRES_HOST").expect("Environment Variable 'POSTGRES_HOST' is unset. Check to see environment variables are being set/loaded correctly."),
                user: env::var("POSTGRES_AUTHORIZED_USER").expect("Environment Variable 'POSTGRES_AUTHORIZED_USER' is unset. Check to see environment variables are being set/loaded correctly."),
                password: env::var("POSTGRES_USER_PASSWORD").expect("Environment Variable 'POSTGRES_USER_PASSWORD' is unset. Check to see environment variables are being set/loaded correctly."),
                port: 5432
            }
        }
    }
}