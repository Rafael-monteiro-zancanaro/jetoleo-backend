#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn init() -> Self {
        let (database_url, server_port) = (
            std::env::var("DATABASE_URL").expect("DATABASE_URL não configurado!"),
            std::env::var("SERVER_PORT").expect("SERVER_PORT não configurado!"),
        );

        return Self {
            database_url,
            server_port: server_port.parse::<u16>().unwrap(),
        };
    }
}
