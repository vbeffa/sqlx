use sqlx_wasm_test::test_type;

use std::str::FromStr;

test_type!(decimal<sqlx::types::Decimal>(Postgres,
    "0::numeric" == sqlx::types::Decimal::from_str("0").unwrap(),
    "1::numeric" == sqlx::types::Decimal::from_str("1").unwrap(),
    "10000::numeric" == sqlx::types::Decimal::from_str("10000").unwrap(),
    "0.1::numeric" == sqlx::types::Decimal::from_str("0.1").unwrap(),
    "0.01234::numeric" == sqlx::types::Decimal::from_str("0.01234").unwrap(),
    "12.34::numeric" == sqlx::types::Decimal::from_str("12.34").unwrap(),
    "12345.6789::numeric" == sqlx::types::Decimal::from_str("12345.6789").unwrap(),
));
