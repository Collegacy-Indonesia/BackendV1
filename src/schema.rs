table! {
    _prisma_migrations (id) {
        id -> Varchar,
        checksum -> Varchar,
        finished_at -> Nullable<Datetime>,
        migration_name -> Varchar,
        logs -> Nullable<Text>,
        rolled_back_at -> Nullable<Datetime>,
        started_at -> Datetime,
        applied_steps_count -> Unsigned<Integer>,
    }
}

table! {
    user (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        email -> Varchar,
        image -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
        email_verified -> Bool,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(_prisma_migrations, user,);
