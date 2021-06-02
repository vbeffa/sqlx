use sqlx::postgres::types::PgInterval;
use sqlx_wasm_test::test_prepared_type;

test_prepared_type!(interval<PgInterval>(
    Postgres,
    "INTERVAL '1h'"
        == PgInterval {
            months: 0,
            days: 0,
            microseconds: 3_600_000_000
        },
    "INTERVAL '-1 hours'"
        == PgInterval {
            months: 0,
            days: 0,
            microseconds: -3_600_000_000
        },
    "INTERVAL '3 months 12 days 1h 15 minutes 10 second '"
        == PgInterval {
            months: 3,
            days: 12,
            microseconds: (3_600 + 15 * 60 + 10) * 1_000_000
        },
    "INTERVAL '03:10:20.116100'"
        == PgInterval {
            months: 0,
            days: 0,
            microseconds: (3 * 3_600 + 10 * 60 + 20) * 1_000_000 + 116100
        },
));
