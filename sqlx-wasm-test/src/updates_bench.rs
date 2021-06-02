use sqlx::Executor;
use sqlx_wasm_test::time_update_query;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn updates_query_small() {
    time_update_query!("small", 100u32);
}

#[wasm_bindgen_test]
async fn updates_query_medium() {
    time_update_query!("medium", 1000u32);
}

#[wasm_bindgen_test]
async fn updates_query_large() {
    time_update_query!("large", 10000u32);
}
