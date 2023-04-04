use {
    crate::miner::*,
    crate::util::*,
    crate::DBPool,
    actix_web::web::{Data, Json, Path},
    actix_web::HttpResponse,
    uuid::Uuid,
};

// list all miners
#[get("/miners")]
pub async fn list_miners(pool: Data<DBPool>) -> HttpResponse {
    let mut conn = crate::get_connection_to_pool(pool);
    let miners: Vec<Miner> = fetch_all_miners(&mut conn);
    ResponseType::Ok(miners).get_response()
}

// get miner by id
#[get("/miners/{id}")]
pub async fn get_miner(path: Path<(String,)>, pool: Data<DBPool>) -> HttpResponse {
    let mut conn = crate::get_connection_to_pool(pool);
    let miner: Option<Miner> =
        get_miner_by_id(Uuid::parse_str(path.0.as_str()).unwrap(), &mut conn);

    match miner {
        Some(miner) => ResponseType::Ok(miner).get_response(),
        None => ResponseType::NotFound(NotFoundMessage::new("Miner not found".to_string()))
            .get_response(),
    }
}

// Create a new miner
#[post("/wallets/{id}/miners")]
pub async fn create_miner(
    path: Path<(String,)>,
    miner_request: Json<NewMinerRequest>,
    pool: Data<DBPool>,
) -> HttpResponse {
    let mut conn = crate::get_connection_to_pool(pool);
    match create_new_miner(
        miner_request.0,
        Uuid::parse_str(path.0.as_str()).unwrap(),
        &mut conn,
    ) {
        Ok(created_miner) => ResponseType::Created(created_miner).get_response(),
        Err(_) => ResponseType::NotFound(NotFoundMessage::new("Error creating miner.".to_string()))
            .get_response(),
    }
}
