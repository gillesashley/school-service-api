use sqlx::postgres::PgPool;

pub struct AppState {
    pub db: PgPool,
    pub host: String,
    pub port: i32,
    pub jwt_secret: String,
}
