use sqlx::postgres::Postgres;
use sqlx::Executor;
use sqlx_test::{new, setup_if_needed, time_update_query};
use std::time::Instant;

// #[sqlx_macros::test]
async fn updates_query_small() {
    time_update_query!("small", 100u32);
}

#[sqlx_macros::test]
async fn updates_query_medium() {
    time_update_query!("medium", 1000u32);
}

// #[sqlx_macros::test]
async fn updates_query_large() {
    time_update_query!("large", 10000u32);
}
