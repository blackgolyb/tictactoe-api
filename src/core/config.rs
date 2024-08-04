pub struct Config {
    pub port: u16,
    pub host: String,
    pub api_version: u16,
    pub assets: String,
    pub db_path: String,
}

pub fn load_config() -> Config {
    Config {
        port: 8080,
        host: "127.0.0.1".into(),
        api_version: 1,
        assets: "assets".into(),
        db_path: "db.db".into(),
    }
}

// const config: Config = load_config();
