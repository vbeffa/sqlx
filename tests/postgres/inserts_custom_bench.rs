use sqlx::postgres::Postgres;
use sqlx::Executor;
use sqlx_test::{new, setup_if_needed, time_insert_query};
use std::time::Instant;

#[sqlx_macros::test]
async fn inserts_query_small() {
    time_insert_query!("small", 100u32);
}

#[sqlx_macros::test]
async fn inserts_query_medium() {
    time_insert_query!("medium", 1000u32);
}

#[sqlx_macros::test]
async fn inserts_query_large() {
    time_insert_query!("large", 10000u32);
}
