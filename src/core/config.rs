pub struct Config {
    pub port: u16,
    pub host: String,
    pub api_version: u16,
    pub assets: String,
    pub db_path: String,
}

pub fn load_config() -> Config {
    let host: String = std::env::var("HOST").expect("HOST must be set");
    let port: u16 = std::env::var("PORT")
        .expect("PORT must be set")
        .parse::<u16>()
        .expect("PORT must be integer number in range [0, 65535]");
    let db_path: String = std::env::var("DB_PATH").expect("DB_PATH must be set");
    let assets_path: String = std::env::var("ASSETS_PATH").expect("ASSETS_PATH must be set");
    let api_version: u16 = std::env::var("API_VERSION")
        .expect("API_VERSION must be set")
        .parse::<u16>()
        .expect("API_VERSION must be in integer number");

    Config {
        port,
        host,
        api_version,
        assets: assets_path,
        db_path,
    }
}
