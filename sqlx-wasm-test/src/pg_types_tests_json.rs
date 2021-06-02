use serde_json::value::RawValue as JsonRawValue;
use serde_json::{json, Value as JsonValue};
use sqlx::postgres::PgRow;
use sqlx::types::Json;
use sqlx::{Executor, Row};
use sqlx_wasm_test::test_type;

// When testing JSON, coerce to JSONB for `=` comparison as `JSON = JSON` is not
// supported in PostgreSQL

test_type!(json<JsonValue>(
    Postgres,
    "SELECT ({0}::jsonb is not distinct from $1::jsonb)::int4, {0} as _2, $2 as _3",
    "'\"Hello, World\"'::json" == json!("Hello, World"),
    "'\"😎\"'::json" == json!("😎"),
    "'\"🙋‍♀️\"'::json" == json!("🙋‍♀️"),
    "'[\"Hello\", \"World!\"]'::json" == json!(["Hello", "World!"])
));

test_type!(json_array<Vec<JsonValue>>(
    Postgres,
    "SELECT ({0}::jsonb[] is not distinct from $1::jsonb[])::int4, {0} as _2, $2 as _3",
    "array['\"😎\"'::json, '\"🙋‍♀️\"'::json]::json[]" == vec![json!("😎"), json!("🙋‍♀️")],
));

test_type!(jsonb<JsonValue>(
    Postgres,
    "'\"Hello, World\"'::jsonb" == json!("Hello, World"),
    "'\"😎\"'::jsonb" == json!("😎"),
    "'\"🙋‍♀️\"'::jsonb" == json!("🙋‍♀️"),
    "'[\"Hello\", \"World!\"]'::jsonb" == json!(["Hello", "World!"])
));

test_type!(jsonb_array<Vec<JsonValue>>(
    Postgres,
    "array['\"😎\"'::jsonb, '\"🙋‍♀️\"'::jsonb]::jsonb[]" == vec![json!("😎"), json!("🙋‍♀️")],
));

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
struct Friend {
    name: String,
    age: u32,
}

test_type!(json_struct<Json<Friend>>(Postgres,
    "'{\"name\":\"Joe\",\"age\":33}'::jsonb" == Json(Friend { name: "Joe".to_string(), age: 33 })
));

test_type!(json_struct_vec<Vec<Json<Friend>>>(Postgres,
    "array['{\"name\":\"Joe\",\"age\":33}','{\"name\":\"Bob\",\"age\":22}']::jsonb[]"
        == vec![
            Json(Friend { name: "Joe".to_string(), age: 33 }),
            Json(Friend { name: "Bob".to_string(), age: 22 }),
        ]
));

#[wasm_bindgen_test::wasm_bindgen_test]
async fn test_json_raw_value() {
    let mut conn = sqlx_wasm_test::new().await;

    // unprepared, text API
    let row: PgRow = conn
        .fetch_one("SELECT '{\"hello\": \"world\"}'::jsonb")
        .await
        .unwrap();

    let value: &JsonRawValue = row.try_get(0).unwrap();

    assert_eq!(value.get(), "{\"hello\": \"world\"}");

    // prepared, binary API
    let row: PgRow = conn
        .fetch_one(sqlx::query("SELECT '{\"hello\": \"world\"}'::jsonb"))
        .await
        .unwrap();

    let value: &JsonRawValue = row.try_get(0).unwrap();

    assert_eq!(value.get(), "{\"hello\": \"world\"}");
}
