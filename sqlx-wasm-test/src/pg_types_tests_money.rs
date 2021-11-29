use sqlx_wasm_test::test_prepared_type;

use sqlx::postgres::types::PgMoney;

test_prepared_type!(money<PgMoney>(Postgres, "123.45::money" == PgMoney(12345)));

test_prepared_type!(money_vec<Vec<PgMoney>>(Postgres,
    "array[123.45,420.00,666.66]::money[]" == vec![PgMoney(12345), PgMoney(42000), PgMoney(66666)],
));
