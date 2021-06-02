use sqlx_wasm_test::test_type;

test_type!(uuid<sqlx::types::Uuid>(Postgres,
    "'b731678f-636f-4135-bc6f-19440c13bd19'::uuid"
        == sqlx::types::Uuid::parse_str("b731678f-636f-4135-bc6f-19440c13bd19").unwrap(),
    "'00000000-0000-0000-0000-000000000000'::uuid"
        == sqlx::types::Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap()
));

test_type!(uuid_vec<Vec<sqlx::types::Uuid>>(Postgres,
    "'{b731678f-636f-4135-bc6f-19440c13bd19,00000000-0000-0000-0000-000000000000}'::uuid[]"
        == vec![
           sqlx::types::Uuid::parse_str("b731678f-636f-4135-bc6f-19440c13bd19").unwrap(),
           sqlx::types::Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap()
        ]
));
