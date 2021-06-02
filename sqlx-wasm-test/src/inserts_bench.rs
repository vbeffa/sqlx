use sqlx::Executor;
use sqlx_wasm_test::time_insert_query;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn insert_query_small() {
    time_insert_query!("small", 100u32);
}

#[wasm_bindgen_test]
async fn insert_query_medium() {
    time_insert_query!("medium", 1000u32);
}

#[wasm_bindgen_test]
async fn insert_query_large() {
    time_insert_query!("large", 10000u32);
}
