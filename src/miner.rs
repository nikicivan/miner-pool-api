use {
    super::schema::miners,
    crate::wallet::*,
    crate::DBPooledConnection,
    diesel::result::Error,
    diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl},
    rand::Rng,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

// ---------- JSON payload (REST)

#[derive(Deserialize, Serialize, Debug)]
pub struct Miner {
    pub id: String,
    pub address: String,
    pub club_name: String,
    pub nickname: String,
    pub hash_rate: i32,
    pub shares_mined: i32,
}

impl Miner {
    pub fn to_miner_dao(&self) -> MinerDao {
        MinerDao {
            id: Uuid::parse_str(self.id.as_str()).unwrap(),
            address: Uuid::parse_str(self.address.as_str()).unwrap(),
            nickname: self.nickname.to_string(),
            hash_rate: self.hash_rate,
            shares_mined: self.shares_mined,
        }
    }
}

// ---------- POST Request Body for new miner

#[derive(Deserialize, Serialize, Debug)]
pub struct NewMinerRequest {
    nickname: String,
}

// ---------- Data Access Object (DAO Table Records)

#[derive(Queryable, Insertable, Debug, PartialEq)]
#[table_name = "miners"]
pub struct MinerDao {
    pub id: Uuid,
    pub address: Uuid,
    pub nickname: String,
    pub hash_rate: i32,
    pub shares_mined: i32,
}

impl MinerDao {
    pub fn to_miner(&self, other_club_name: String) -> Miner {
        Miner {
            id: self.id.to_string(),
            address: self.address.to_string(),
            club_name: other_club_name.to_string(),
            nickname: self.nickname.to_string(),
            hash_rate: self.hash_rate,
            shares_mined: self.shares_mined,
        }
    }
}

// pub fn get_club_name(_address: Uuid, conn: &mut DBPooledConnection) -> String {
//     match fetch_wallet_by_id(_address, conn) {
//         Some(matched_wallet) => matched_wallet.club_name,
//         None => "Club name not found".to_string(),
//     }
// }

pub fn fetch_all_miners(conn: &mut DBPooledConnection) -> Vec<Miner> {
    use crate::schema::miners::dsl::*;
    use crate::schema::wallets::dsl::*;

    match wallets
        .inner_join(miners)
        .load::<(WalletDAO, MinerDao)>(conn)
    {
        Ok(result) => result
            .into_iter()
            .map(|(w, m)| m.to_miner(w.club_name))
            .collect::<Vec<Miner>>(),
        Err(_) => vec![],
    }
}

pub fn get_miner_by_id(_id: Uuid, conn: &mut DBPooledConnection) -> Option<Miner> {
    use crate::schema::miners::dsl::*;
    use crate::schema::wallets::dsl::*;

    match wallets
        .inner_join(miners)
        .filter(id.eq(_id))
        .load::<(WalletDAO, MinerDao)>(conn)
    {
        Ok(result) => match result.first() {
            Some((w, m)) => Some(m.to_miner(w.club_name.clone())),
            _ => None,
        },
        Err(_) => None,
    }
}

pub fn create_new_miner(
    new_miner_request: NewMinerRequest,
    _address: Uuid,
    conn: &mut DBPooledConnection,
) -> Result<Miner, Error> {
    use crate::schema::miners::dsl::*;

    let new_miner_dao = MinerDao {
        id: Uuid::new_v4(),
        address: _address,
        nickname: new_miner_request.nickname,
        hash_rate: rand::thread_rng().gen_range(20..100),
        shares_mined: rand::thread_rng().gen_range(1..40),
    };

    match diesel::insert_into(miners)
        .values(&new_miner_dao)
        .execute(conn)
    {
        Ok(_) => match get_miner_by_id(new_miner_dao.id, conn) {
            Some(result) => Ok(result),
            None => Err(Error::NotFound),
        },
        Err(e) => Err(e),
    }
}
