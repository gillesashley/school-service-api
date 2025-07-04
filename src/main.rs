use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPool;
use std::env;
use std::io;

pub mod db_access;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod state;

use routes::course_routes;
use state::AppState;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        db: db_pool,
        host: "127.0.0.1".to_string(),
        port: 8000,
        jwt_secret: "your-secret".to_string(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(course_routes)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
