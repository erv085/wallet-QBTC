use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod wallet;
use wallet::QuantumWallet;

#[derive(Deserialize)]
struct RestoreRequest {
    mnemonic: String,
}

#[derive(Serialize)]
struct WalletResponse {
    address: String,
    public_key: String,
}

async fn health_check() -> &'static str {
    "QBTC Post-Quantum Wallet Node Online!"
}

async fn restore_wallet(
    Json(payload): Json<RestoreRequest>,
) -> Result<Json<WalletResponse>, (axum::http::StatusCode, String)> {
    let wallet = QuantumWallet::restore_from_mnemonic(&payload.mnemonic)
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, e))?;

    Ok(Json(WalletResponse {
        address: wallet.qbtc_address,
        public_key: hex::encode(wallet.public_key),
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/wallet/restore", post(restore_wallet))
        .layer(CorsLayer::permissive());

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT harus berupa angka");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("[INFO] QBTC Wallet API berjalan di http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
