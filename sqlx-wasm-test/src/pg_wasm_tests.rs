use futures::TryStreamExt;
use serde_json;
use sqlx::postgres::{PgDatabaseError, PgErrorPosition, PgRow, PgSeverity, Postgres};
use sqlx::{Column, Connection, Executor, Row, Statement, TypeInfo};
use sqlx_wasm_test::new;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn select_query_count() {
    let mut conn = new().await;

    let count = sqlx::query(
        "SELECT count(*) from generate_series(1,100) AS id, md5(random()::text) AS descr",
    )
    .try_map(|row: PgRow| row.try_get::<i64, _>(0))
    .fetch_one(&mut conn)
    .await
    .unwrap();

    assert_eq!(count, 100i64);
}

#[wasm_bindgen_test]
async fn it_executes() {
    let mut conn = new().await;

    let _ = conn
        .execute(
            r#"
CREATE TEMPORARY TABLE users (id INTEGER PRIMARY KEY);
            "#,
        )
        .await;

    for index in 1..=10_i32 {
        let done = sqlx::query("INSERT INTO users (id) VALUES ($1)")
            .bind(index)
            .execute(&mut conn)
            .await
            .unwrap();

        assert_eq!(done.rows_affected(), 1);
    }

    let sum: i32 = sqlx::query("SELECT id FROM users")
        .try_map(|row: PgRow| row.try_get::<i32, _>(0))
        .fetch(&mut conn)
        .try_fold(0_i32, |acc, x| async move { Ok(acc + x) })
        .await
        .unwrap();

    assert_eq!(sum, 55);
}

#[wasm_bindgen_test]
async fn it_can_inspect_errors() {
    let mut conn = new().await;

    let res: Result<_, sqlx::Error> = sqlx::query("select f").execute(&mut conn).await;
    let err = res.unwrap_err();

    // can also do [as_database_error] or use `match ..`
    let err = err.into_database_error().unwrap();

    assert_eq!(err.message(), "column \"f\" does not exist");
    assert_eq!(err.code().as_deref(), Some("42703"));

    // can also do [downcast_ref]
    let err: Box<PgDatabaseError> = err.downcast();

    assert_eq!(err.severity(), PgSeverity::Error);
    assert_eq!(err.message(), "column \"f\" does not exist");
    assert_eq!(err.code(), "42703");
    assert_eq!(err.position(), Some(PgErrorPosition::Original(8)));
    assert_eq!(err.routine(), Some("errorMissingColumn"));
    assert_eq!(err.constraint(), None);
}

#[wasm_bindgen_test]
async fn it_can_inspect_constraint_errors() {
    let mut conn = new().await;

    let _ = conn
        .execute(
            r#"
                CREATE TABLE products (
                    product_no INTEGER,
                    name TEXT,
                    price NUMERIC CHECK (price > 0)
                );
            "#,
        )
        .await;

    let res: Result<_, sqlx::Error> =
        sqlx::query("INSERT INTO products VALUES (1, 'Product 1', 0);")
            .execute(&mut conn)
            .await;
    let err = res.unwrap_err();

    // can also do [as_database_error] or use `match ..`
    let err = err.into_database_error().unwrap();

    assert_eq!(
        err.message(),
        "new row for relation \"products\" violates check constraint \"products_price_check\""
    );
    assert_eq!(err.code().as_deref(), Some("23514"));

    // can also do [downcast_ref]
    let err: Box<PgDatabaseError> = err.downcast();

    assert_eq!(err.severity(), PgSeverity::Error);
    assert_eq!(
        err.message(),
        "new row for relation \"products\" violates check constraint \"products_price_check\""
    );
    assert_eq!(err.code(), "23514");
    assert_eq!(err.position(), None);
    assert_eq!(err.routine(), Some("ExecConstraints"));
    assert_eq!(err.constraint(), Some("products_price_check"));
}

#[wasm_bindgen_test]
async fn it_can_prepare_then_execute() {
    let mut conn = new().await;
    let _ = conn
        .execute(
            "create temp table tweet (id  BIGSERIAL PRIMARY KEY,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                text       TEXT        NOT NULL,
                owner_id   BIGINT
            )",
        )
        .await;
    let mut tx = conn.begin().await.unwrap();

    let tweet_id: i64 =
        sqlx::query_scalar("INSERT INTO tweet (text) VALUES ( 'Hello, World' ) RETURNING id")
            .fetch_one(&mut tx)
            .await
            .unwrap();

    let statement = tx
        .prepare("SELECT * FROM tweet WHERE id = $1")
        .await
        .unwrap();

    assert_eq!(statement.column(0).name(), "id");
    assert_eq!(statement.column(1).name(), "created_at");
    assert_eq!(statement.column(2).name(), "text");
    assert_eq!(statement.column(3).name(), "owner_id");

    assert_eq!(statement.column(0).type_info().name(), "INT8");
    assert_eq!(statement.column(1).type_info().name(), "TIMESTAMPTZ");
    assert_eq!(statement.column(2).type_info().name(), "TEXT");
    assert_eq!(statement.column(3).type_info().name(), "INT8");

    let row = statement
        .query()
        .bind(tweet_id)
        .fetch_one(&mut tx)
        .await
        .unwrap();
    let tweet_text: &str = row.try_get("text").unwrap();

    assert_eq!(tweet_text, "Hello, World");
}

#[wasm_bindgen_test]
async fn it_supports_domain_types_in_composite_domain_types() {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct MonthId(i16);

    impl sqlx::Type<Postgres> for MonthId {
        fn type_info() -> sqlx::postgres::PgTypeInfo {
            sqlx::postgres::PgTypeInfo::with_name("month_id")
        }

        fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
            *ty == Self::type_info()
        }
    }

    impl<'r> sqlx::Decode<'r, Postgres> for MonthId {
        fn decode(
            value: sqlx::postgres::PgValueRef<'r>,
        ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
            Ok(Self(
                <i16 as sqlx::Decode<Postgres>>::decode(value).unwrap(),
            ))
        }
    }

    impl<'q> sqlx::Encode<'q, Postgres> for MonthId {
        fn encode_by_ref(
            &self,
            buf: &mut sqlx::postgres::PgArgumentBuffer,
        ) -> sqlx::encode::IsNull {
            self.0.encode(buf)
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct WinterYearMonth {
        year: i32,
        month: MonthId,
    }

    impl sqlx::Type<Postgres> for WinterYearMonth {
        fn type_info() -> sqlx::postgres::PgTypeInfo {
            sqlx::postgres::PgTypeInfo::with_name("winter_year_month")
        }

        fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
            *ty == Self::type_info()
        }
    }

    impl<'r> sqlx::Decode<'r, Postgres> for WinterYearMonth {
        fn decode(
            value: sqlx::postgres::PgValueRef<'r>,
        ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
            let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value).unwrap();

            let year = decoder.try_decode::<i32>().unwrap();
            let month = decoder.try_decode::<MonthId>().unwrap();

            Ok(Self { year, month })
        }
    }

    impl<'q> sqlx::Encode<'q, Postgres> for WinterYearMonth {
        fn encode_by_ref(
            &self,
            buf: &mut sqlx::postgres::PgArgumentBuffer,
        ) -> sqlx::encode::IsNull {
            let mut encoder = sqlx::postgres::types::PgRecordEncoder::new(buf);
            encoder.encode(self.year);
            encoder.encode(self.month);
            encoder.finish();
            sqlx::encode::IsNull::No
        }
    }

    let mut conn = new().await;

    let _ = conn
        .execute(
            "
            CREATE temp TABLE heating_bills (
              month winter_year_month NOT NULL PRIMARY KEY,
              cost INT4 NOT NULL
            )",
        )
        .await;

    {
        let result = sqlx::query("DELETE FROM heating_bills;")
            .execute(&mut conn)
            .await;

        let result = result.unwrap();
        assert_eq!(result.rows_affected(), 0);
    }

    {
        let result = sqlx::query(
            "INSERT INTO heating_bills(month, cost) VALUES($1::winter_year_month, 100);",
        )
        .bind(WinterYearMonth {
            year: 2021,
            month: MonthId(1),
        })
        .execute(&mut conn)
        .await;

        let result = result.unwrap();
        assert_eq!(result.rows_affected(), 1);
    }

    {
        let result = sqlx::query("DELETE FROM heating_bills;")
            .execute(&mut conn)
            .await;

        let result = result.unwrap();
        assert_eq!(result.rows_affected(), 1);
    }
}

#[wasm_bindgen_test]
async fn it_describes_and_inserts_json_and_jsonb() {
    let mut conn = new().await;

    let _ = conn
        .execute(
            r#"
                CREATE TEMPORARY TABLE json_stuff (obj json, obj2 jsonb);
            "#,
        )
        .await;

    let query = "INSERT INTO json_stuff (obj, obj2) VALUES ($1, $2)";
    let _ = conn.describe(query).await;

    let done = sqlx::query(query)
        .bind(serde_json::json!({ "a": "a" }))
        .bind(serde_json::json!({ "a": "a" }))
        .execute(&mut conn)
        .await
        .unwrap();

    assert_eq!(done.rows_affected(), 1);
}

#[wasm_bindgen_test]
async fn test_listener_cleanup() {
    use sqlx::postgres::PgListener;

    // Create a connection on which to send notifications
    let mut notify_conn = new().await;

    let mut listener = PgListener::connect(sqlx_wasm_test::URL).await.unwrap();
    assert!(listener.listen("test_channel").await.is_ok());

    notify_conn.execute("NOTIFY test_channel").await.unwrap();

    assert!(
        listener.recv().await.is_ok(),
        "Notification sent and received"
    );
}
