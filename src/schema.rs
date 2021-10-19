table! {
    User (id) {
        id -> Varchar,
        name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        emailVerified -> Nullable<Datetime>,
        image -> Nullable<Varchar>,
        createdAt -> Datetime,
        updatedAt -> Datetime,
    }
}

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

allow_tables_to_appear_in_same_query!(
    User,
    _prisma_migrations,
);
