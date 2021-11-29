use sqlx_wasm_test::{test_prepared_type, test_type};

use sqlx::types::time::{Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};
use time::{date, time};

type PgTimeTz = sqlx::postgres::types::PgTimeTz<Time, UtcOffset>;

test_type!(time_date<Date>(
    Postgres,
    "DATE '2001-01-05'" == date!(2001 - 1 - 5),
    "DATE '2050-11-23'" == date!(2050 - 11 - 23)
));

test_type!(time_time<Time>(
    Postgres,
    "TIME '05:10:20.115100'" == time!(5:10:20.115100)
));

test_type!(time_date_time<PrimitiveDateTime>(
    Postgres,
    "TIMESTAMP '2019-01-02 05:10:20'" == date!(2019 - 1 - 2).with_time(time!(5:10:20)),
    "TIMESTAMP '2019-01-02 05:10:20.115100'"
        == date!(2019 - 1 - 2).with_time(time!(5:10:20.115100))
));

test_type!(time_timestamp<OffsetDateTime>(
    Postgres,
    "TIMESTAMPTZ '2019-01-02 05:10:20.115100'"
        == date!(2019 - 1 - 2)
            .with_time(time!(5:10:20.115100))
            .assume_utc()
));

test_prepared_type!(time_time_tz<PgTimeTz>(Postgres,
    "TIMETZ '05:10:20.115100+00'" == PgTimeTz { time: time!(5:10:20.115100), offset: UtcOffset::east_seconds(0) },
    "TIMETZ '05:10:20.115100+06:30'" == PgTimeTz { time: time!(5:10:20.115100), offset: UtcOffset::east_seconds(60 * 60 * 6 + 1800) },
    "TIMETZ '05:10:20.115100-05'" == PgTimeTz { time: time!(5:10:20.115100), offset: UtcOffset::west_seconds(60 * 60 * 5) },
    "TIMETZ '05:10:20+02'" == PgTimeTz { time: time!(5:10:20), offset: UtcOffset::east_seconds(60 * 60 * 2 )}
));
