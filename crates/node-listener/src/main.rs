use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};

use omnistacks_data::{
    db::{self, *},
    db_pool::{self, ConnectionPool},
    models::NewNodeMessage,
    schema::node_messages,
};
use serde_json::{json, Value};
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "DEBUG".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_pool: ConnectionPool = db_pool::get_pool().expect("Failed to get DB pool");

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/", post(index_handler))
        .route("/new_burn_block", post(new_burn_block_handler))
        .route("/new_block", post(new_block_handler))
        .route("/new_mempool_tx", post(new_mempool_tx_handler))
        .route("/drop_mempool_tx", post(drop_mempool_tx_handler))
        .route("/attachments/new", post(new_attachments_handler))
        .route("/new_microblocks", post(new_microblocks_handler))
        .layer(Extension(db_pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn save_message(message_type: &str, body: Value, pool: ConnectionPool) -> impl IntoResponse {
    match pool.get() {
        Ok(conn) => {
            let new_node_message = NewNodeMessage {
                message_type,
                body: &body,
            };

            let insert_result = db::insert_into(node_messages::table)
                .values(new_node_message)
                .execute(&conn);

            match insert_result {
                Ok(_) => {
                    let response = format!("Successfully saved new '{}' message", message_type);
                    info!("{}", response);
                    (StatusCode::OK, Json(json!({ "OK": response })))
                }
                Err(e) => {
                    let error_msg = format!("Failed to insert new node message: {}", e);
                    error!("{}", error_msg);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!(error_msg)))
                }
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!("Failed to obtain DB connection")),
        ),
    }
}

async fn index_handler() -> impl IntoResponse {
    info!("Processing Route: /");
    (StatusCode::OK, Json(json!({ "ok": "It's alive" })))
}

async fn new_burn_block_handler(
    Json(body): Json<Value>,
    Extension(pool): Extension<ConnectionPool>,
) -> impl IntoResponse {
    save_message("new_burn_block", body, pool).await
}

async fn new_block_handler(
    Json(body): Json<Value>,
    Extension(pool): Extension<ConnectionPool>,
) -> impl IntoResponse {
    save_message("new_block", body, pool).await
}

async fn new_mempool_tx_handler(
    Json(body): Json<Value>,
    Extension(pool): Extension<ConnectionPool>,
) -> impl IntoResponse {
    save_message("new_mempool_tx", body, pool).await
}

async fn drop_mempool_tx_handler(
    Json(body): Json<Value>,
    Extension(pool): Extension<ConnectionPool>,
) -> impl IntoResponse {
    save_message("drop_mempool_tx", body, pool).await
}

async fn new_attachments_handler(
    Json(body): Json<Value>,
    Extension(pool): Extension<ConnectionPool>,
) -> impl IntoResponse {
    save_message("new_attachments", body, pool).await
}

async fn new_microblocks_handler(
    Json(body): Json<Value>,
    Extension(pool): Extension<ConnectionPool>,
) -> impl IntoResponse {
    save_message("new_microblocks", body, pool).await
}
