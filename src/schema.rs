// @generated automatically by Diesel CLI.

diesel::table! {
    miners (id) {
        id -> Uuid,
        address -> Uuid,
        nickname -> Text,
        hash_rate -> Int4,
        shares_mined -> Int4,
    }
}

diesel::table! {
    wallets (address) {
        address -> Uuid,
        club_name -> Text,
    }
}

diesel::joinable!(miners -> wallets (address));

diesel::allow_tables_to_appear_in_same_query!(
    miners,
    wallets,
);
