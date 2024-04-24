pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().expect("Failed to read .env file");

        Self {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL is not set"),
        }
    }
}
