table! {
    summit_registrant (id) {
        id -> Integer,
        user_id -> Integer,
        no_hp -> Varchar,
        universitas -> Varchar,
        jurusan -> Varchar,
        batch -> Integer,
        role -> Varchar,
        link_drive -> Varchar,
    }
}

table! {
    user (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        email -> Varchar,
        password -> Varchar,
        email_verified -> Bool,
        image -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

allow_tables_to_appear_in_same_query!(
    summit_registrant,
    user,
);
