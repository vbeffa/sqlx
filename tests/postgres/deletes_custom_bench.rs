use sqlx::postgres::Postgres;
use sqlx::Executor;
use sqlx_test::{new, setup_if_needed, time_delete_query};
use std::time::Instant;

async fn deletes_query_small() {
    time_delete_query!("small", 100u32);
}

#[sqlx_macros::test]
async fn deletes_query_medium() {
    time_delete_query!("medium", 1000u32);
}

async fn deletes_query_large() {
    time_delete_query!("large", 10000u32);
}
