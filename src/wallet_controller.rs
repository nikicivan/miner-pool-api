use actix_web::web::Path;

use {
    crate::util::*,
    crate::wallet::*,
    crate::DBPool,
    actix_web::web::{Data, Json},
    actix_web::HttpResponse,
    uuid::Uuid,
};

// list all wallets
#[get("/wallets")]
pub async fn list_wallets(pool: Data<DBPool>) -> HttpResponse {
    let mut conn = crate::get_connection_to_pool(pool);
    let wallets: Vec<Wallet> = fetch_all_wallets(&mut conn);
    ResponseType::Ok(wallets).get_response()
}

// Get a wallet by Id
#[get("wallets/{id}")]
pub async fn get_wallet(path: Path<(String,)>, pool: Data<DBPool>) -> HttpResponse {
    let mut conn = crate::get_connection_to_pool(pool);
    let wallet: Option<Wallet> =
        fetch_wallet_by_id(Uuid::parse_str(path.0.as_str()).unwrap(), &mut conn);
    match wallet {
        Some(wallet) => ResponseType::Ok(wallet).get_response(),
        None => ResponseType::NotFound(NotFoundMessage::new("Wallet/Club not found".to_string()))
            .get_response(),
    }
}

// Create a new wallet
#[post("/wallets")]
pub async fn create_wallet(
    wallet_request: Json<NewWalletRequest>,
    pool: Data<DBPool>,
) -> HttpResponse {
    let mut conn = crate::get_connection_to_pool(pool);
    match create_new_wallet(wallet_request.0, &mut conn) {
        Ok(created_wallet) => ResponseType::Ok(created_wallet).get_response(),
        Err(_) => ResponseType::NotFound(NotFoundMessage::new("Error creating wallet".to_string()))
            .get_response(),
    }
}
