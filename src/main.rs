use std::env;
use std::sync::Arc;
use axum::http::{HeaderValue, Method};
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;

use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use rust_axum_example::api::router::create_router;
use rust_axum_example::server::state::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();
    /*tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();*/

    Arc::new(env::var("JWT_SECRET").expect("JWT_SECRET must be set"));
    let environment = env::var("ENVIRONMENT").expect("ENVIRONMENT must be set");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState { db: pool.clone()  }), &environment)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
