#[derive(Debug, Clone)]
pub struct ApplicationConfiguration {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
    pub redis_url: String,
    pub redis_secret_key: String,
}

impl ApplicationConfiguration {
    pub fn init() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in = std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");

        let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
        let redis_secret_key =
            std::env::var("REDIS_SECRET_KEY").expect("REDIS_SECRET_KEY must be set");

        Self {
            database_url,
            jwt_secret,
            jwt_expires_in,
            jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
            redis_url,
            redis_secret_key,
        }
    }
}
