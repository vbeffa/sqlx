use sqlx_wasm_test::time_query;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn select_query_small() {
    time_query!(
        "small",
        "SELECT generate_series(1,100) AS id, md5(random()::text) AS descr"
    );
}

#[wasm_bindgen_test]
async fn select_query_medium() {
    time_query!(
        "medium",
        "SELECT generate_series(1,1000) AS id, md5(random()::text) AS descr"
    );
}

#[wasm_bindgen_test]
async fn select_query_large() {
    time_query!(
        "large",
        "SELECT generate_series(1,10000) AS id, md5(random()::text) AS descr"
    );
}
