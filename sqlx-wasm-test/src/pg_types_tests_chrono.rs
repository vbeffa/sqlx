use sqlx::types::chrono::{
    DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc,
};
use sqlx_wasm_test::test_type;

type PgTimeTz = sqlx::postgres::types::PgTimeTz<NaiveTime, FixedOffset>;

test_type!(chrono_date<NaiveDate>(Postgres,
    "DATE '2001-01-05'" == NaiveDate::from_ymd(2001, 1, 5),
    "DATE '2050-11-23'" == NaiveDate::from_ymd(2050, 11, 23)
));

test_type!(chrono_time<NaiveTime>(Postgres,
    "TIME '05:10:20.115100'" == NaiveTime::from_hms_micro(5, 10, 20, 115100)
));

test_type!(chrono_date_time<NaiveDateTime>(Postgres,
    "'2019-01-02 05:10:20'::timestamp" == NaiveDate::from_ymd(2019, 1, 2).and_hms(5, 10, 20)
));

test_type!(chrono_date_time_vec<Vec<NaiveDateTime>>(Postgres,
    "array['2019-01-02 05:10:20']::timestamp[]"
        == vec![NaiveDate::from_ymd(2019, 1, 2).and_hms(5, 10, 20)]
));

test_type!(chrono_date_time_tz_utc<DateTime::<Utc>>(Postgres,
    "TIMESTAMPTZ '2019-01-02 05:10:20.115100'"
        == DateTime::<Utc>::from_utc(
            NaiveDate::from_ymd(2019, 1, 2).and_hms_micro(5, 10, 20, 115100),
            Utc,
        )
));

test_type!(chrono_date_time_tz<DateTime::<FixedOffset>>(Postgres,
    "TIMESTAMPTZ '2019-01-02 05:10:20.115100+06:30'"
        == FixedOffset::east(60 * 60 * 6 + 1800).ymd(2019, 1, 2).and_hms_micro(5, 10, 20, 115100)
));

test_type!(chrono_date_time_tz_vec<Vec<DateTime::<Utc>>>(Postgres,
    "array['2019-01-02 05:10:20.115100']::timestamptz[]"
        == vec![
            DateTime::<Utc>::from_utc(
                NaiveDate::from_ymd(2019, 1, 2).and_hms_micro(5, 10, 20, 115100),
                Utc,
            )
        ]
));

test_type!(chrono_time_tz<PgTimeTz>(Postgres,
    "TIMETZ '05:10:20.115100+00'" == PgTimeTz { time: NaiveTime::from_hms_micro(5, 10, 20, 115100), offset: FixedOffset::east(0) },
    "TIMETZ '05:10:20.115100+06:30'" == PgTimeTz { time: NaiveTime::from_hms_micro(5, 10, 20, 115100), offset: FixedOffset::east(60 * 60 * 6 + 1800) },
    "TIMETZ '05:10:20.115100-05'" == PgTimeTz { time: NaiveTime::from_hms_micro(5, 10, 20, 115100), offset: FixedOffset::west(60 * 60 * 5) },
    "TIMETZ '05:10:20+02'" == PgTimeTz { time: NaiveTime::from_hms(5, 10, 20), offset: FixedOffset::east(60 * 60 * 2 )}
));
