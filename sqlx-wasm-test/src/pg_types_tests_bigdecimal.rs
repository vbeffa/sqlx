use sqlx_wasm_test::test_type;

test_type!(bigdecimal<sqlx::types::BigDecimal>(Postgres,

    // https://github.com/launchbadge/sqlx/issues/283
    "0::numeric" == "0".parse::<sqlx::types::BigDecimal>().unwrap(),

    "1::numeric" == "1".parse::<sqlx::types::BigDecimal>().unwrap(),
    "10000::numeric" == "10000".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.1::numeric" == "0.1".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.01::numeric" == "0.01".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.012::numeric" == "0.012".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.0123::numeric" == "0.0123".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.01234::numeric" == "0.01234".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.012345::numeric" == "0.012345".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.0123456::numeric" == "0.0123456".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.01234567::numeric" == "0.01234567".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.012345678::numeric" == "0.012345678".parse::<sqlx::types::BigDecimal>().unwrap(),
    "0.0123456789::numeric" == "0.0123456789".parse::<sqlx::types::BigDecimal>().unwrap(),
    "12.34::numeric" == "12.34".parse::<sqlx::types::BigDecimal>().unwrap(),
    "12345.6789::numeric" == "12345.6789".parse::<sqlx::types::BigDecimal>().unwrap(),
));
