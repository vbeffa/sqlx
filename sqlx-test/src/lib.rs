use sqlx::pool::PoolOptions;
use sqlx::{Connection, Database, Pool};
use std::env;

pub fn setup_if_needed() {
    let _ = dotenv::dotenv();
    let _ = env_logger::builder().is_test(true).try_init();
}

// Make a new connection
// Ensure [dotenv] and [env_logger] have been setup
pub async fn new<DB>() -> anyhow::Result<DB::Connection>
where
    DB: Database,
{
    setup_if_needed();

    Ok(DB::Connection::connect(&env::var("DATABASE_URL")?).await?)
}

// Make a new pool
// Ensure [dotenv] and [env_logger] have been setup
pub async fn pool<DB>() -> anyhow::Result<Pool<DB>>
where
    DB: Database,
{
    setup_if_needed();

    let pool = PoolOptions::<DB>::new()
        .min_connections(0)
        .max_connections(5)
        .test_before_acquire(true)
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    Ok(pool)
}

// Test type encoding and decoding
#[macro_export]
macro_rules! test_type {
    ($name:ident<$ty:ty>($db:ident, $sql:literal, $($text:literal == $value:expr),+ $(,)?)) => {
        $crate::__test_prepared_type!($name<$ty>($db, $sql, $($text == $value),+));
        $crate::test_unprepared_type!($name<$ty>($db, $($text == $value),+));
    };

    ($name:ident<$ty:ty>($db:ident, $($text:literal == $value:expr),+ $(,)?)) => {
        paste::item! {
            $crate::__test_prepared_type!($name<$ty>($db, $crate::[< $db _query_for_test_prepared_type >]!(), $($text == $value),+));
            $crate::test_unprepared_type!($name<$ty>($db, $($text == $value),+));
        }
    };

    ($name:ident($db:ident, $($text:literal == $value:expr),+ $(,)?)) => {
        $crate::test_type!($name<$name>($db, $($text == $value),+));
    };
}

// Test type decoding only
#[macro_export]
macro_rules! test_decode_type {
    ($name:ident<$ty:ty>($db:ident, $($text:literal == $value:expr),+ $(,)?)) => {
        $crate::__test_prepared_decode_type!($name<$ty>($db, $($text == $value),+));
        $crate::test_unprepared_type!($name<$ty>($db, $($text == $value),+));
    };

    ($name:ident($db:ident, $($text:literal == $value:expr),+ $(,)?)) => {
        $crate::test_decode_type!($name<$name>($db, $($text == $value),+));
    };
}

// Test type encoding and decoding
#[macro_export]
macro_rules! test_prepared_type {
    ($name:ident<$ty:ty>($db:ident, $sql:literal, $($text:literal == $value:expr),+ $(,)?)) => {
        $crate::__test_prepared_type!($name<$ty>($db, $sql, $($text == $value),+));
    };

    ($name:ident<$ty:ty>($db:ident, $($text:literal == $value:expr),+ $(,)?)) => {
        paste::item! {
            $crate::__test_prepared_type!($name<$ty>($db, $crate::[< $db _query_for_test_prepared_type >]!(), $($text == $value),+));
        }
    };

    ($name:ident($db:ident, $($text:literal == $value:expr),+ $(,)?)) => {
        $crate::__test_prepared_type!($name<$name>($db, $($text == $value),+));
    };
}

// Test type decoding for the simple (unprepared) query API
#[macro_export]
macro_rules! test_unprepared_type {
    ($name:ident<$ty:ty>($db:ident, $($text:literal == $value:expr),+ $(,)?)) => {
        paste::item! {
            #[sqlx_macros::test]
            async fn [< test_unprepared_type_ $name >] () -> anyhow::Result<()> {
                use sqlx::prelude::*;
                use futures::TryStreamExt;

                let mut conn = sqlx_test::new::<$db>().await?;

                $(
                    let query = format!("SELECT {}", $text);
                    let mut s = conn.fetch(&*query);
                    let row = s.try_next().await?.unwrap();
                    let rec = row.try_get::<$ty, _>(0)?;

                    assert!($value == rec);

                    drop(s);
                )+

                Ok(())
            }
        }
    }
}

// Test type decoding only for the prepared query API
#[macro_export]
macro_rules! __test_prepared_decode_type {
    ($name:ident<$ty:ty>($db:ident, $($text:literal == $value:expr),+ $(,)?)) => {
        paste::item! {
            #[sqlx_macros::test]
            async fn [< test_prepared_decode_type_ $name >] () -> anyhow::Result<()> {
                use sqlx::Row;

                let mut conn = sqlx_test::new::<$db>().await?;

                $(
                    let query = format!("SELECT {}", $text);

                    let row = sqlx::query(&query)
                        .fetch_one(&mut conn)
                        .await?;

                    let rec: $ty = row.try_get(0)?;

                    assert!($value == rec);
                )+

                Ok(())
            }
        }
    };
}

// Test type encoding and decoding for the prepared query API
#[macro_export]
macro_rules! __test_prepared_type {
    ($name:ident<$ty:ty>($db:ident, $sql:expr, $($text:literal == $value:expr),+ $(,)?)) => {
        paste::item! {
            #[sqlx_macros::test]
            async fn [< test_prepared_type_ $name >] () -> anyhow::Result<()> {
                use sqlx::Row;

                let mut conn = sqlx_test::new::<$db>().await?;

                $(
                    let query = format!($sql, $text);

                    let row = sqlx::query(&query)
                        .bind($value)
                        .bind($value)
                        .fetch_one(&mut conn)
                        .await?;

                    let matches: i32 = row.try_get(0)?;
                    let returned: $ty = row.try_get(1)?;
                    let round_trip: $ty = row.try_get(2)?;

                    assert!(matches != 0,
                            "[1] DB value mismatch; given value: {:?}\n\
                             as returned: {:?}\n\
                             round-trip: {:?}",
                            $value, returned, round_trip);

                    assert_eq!($value, returned,
                            "[2] DB value mismatch; given value: {:?}\n\
                                     as returned: {:?}\n\
                                     round-trip: {:?}",
                                    $value, returned, round_trip);

                    assert_eq!($value, round_trip,
                            "[3] DB value mismatch; given value: {:?}\n\
                                     as returned: {:?}\n\
                                     round-trip: {:?}",
                                    $value, returned, round_trip);
                )+

                Ok(())
            }
        }
    };
}

#[macro_export]
macro_rules! MySql_query_for_test_prepared_type {
    () => {
        "SELECT {0} <=> ?, {0}, ?"
    };
}

#[macro_export]
macro_rules! Mssql_query_for_test_prepared_type {
    () => {
        "SELECT CASE WHEN {0} IS NULL AND @p1 IS NULL THEN 1 WHEN {0} = @p1 THEN 1 ELSE 0 END, {0}, @p2"
    };
}

#[macro_export]
macro_rules! Sqlite_query_for_test_prepared_type {
    () => {
        "SELECT {0} is ?, {0}, ?"
    };
}

#[macro_export]
macro_rules! Postgres_query_for_test_prepared_type {
    () => {
        "SELECT ({0} is not distinct from $1)::int4, {0}, $2"
    };
}

#[macro_export]
macro_rules! time_delete_query {
    ($n:expr, $count:literal) => {
        let mut conn = new::<Postgres>().await.unwrap();

        conn.execute("create temp table bench_deletes (id integer, descr text, primary key(id))")
            .await;

        conn.execute("create bitmap index id_idx on bench_deletes (id)")
            .await;

        let _ = sqlx::query(&format!(
            "insert into bench_deletes (id, descr) select generate_series(1,{}) AS id, md5(random()::text) AS descr",
            $count
        ))

        .execute(&mut conn)
        .await;

        let start = Instant::now();
        for _ in 0..3u8 {
            for i in 1..$count {
                let _ = sqlx::query(&format!(
                    "delete from bench_deletes where id = {}",
                    i
                ))
                .execute(&mut conn)
                .await;
            }
        }

        let end = Instant::now();

        println!("{}: Avg time is {}", $n, end.duration_since(start).as_millis() / 3u128);
    };
}

#[macro_export]
macro_rules! time_update_query {
    ($n:expr, $count:literal) => {
        let mut conn = new::<Postgres>().await.unwrap();

        conn.execute("create temp table bench_updates (id integer, descr text, primary key(id))")
            .await;

        conn.execute("create bitmap index id_idx on bench_updates (id)")
            .await;

        let _ = sqlx::query(&format!(
            "insert into bench_updates (id, descr) select generate_series(1,{}) AS id, md5(random()::text) AS descr",
            $count
        ))
        .execute(&mut conn)
        .await;

        let start = Instant::now();
        for _ in 0..3u8 {
            for i in 1..$count {
                let _ = sqlx::query(&format!(
                    "update bench_updates set descr = md5(random()::text) where id = {}",
                    i
                ))
                .execute(&mut conn)
                .await;
            }
        }

        let end = Instant::now();
        println!("{}: Avg time is {}", $n, end.duration_since(start).as_millis() / 3u128);
    };
}

#[macro_export]
macro_rules! time_insert_query {
    ($n:expr, $count:literal) => {
        let mut conn = new::<Postgres>().await.unwrap();
        conn.execute("create temp table bench_inserts (id integer, descr text)")
            .await;

        let start = Instant::now();

        for _ in 0..3u8 {
            for i in 0..$count {
                let _ = sqlx::query(&format!(
                    "insert into bench_inserts (id, desc) values ({}, md5(random()::text))",
                    i
                ))
                .execute(&mut conn)
                .await;
            }
        }

        let end = Instant::now();
        println!(
            "{}: Avg time is {}",
            $n,
            end.duration_since(start).as_millis() / 3u128
        );
    };
}

#[macro_export]
macro_rules! time_query {
    ($n:expr, $q:expr) => {
        let mut conn = new::<Postgres>().await.unwrap();

        let start = Instant::now();

        for _ in 0..3u8 {
            let _ = sqlx::query($q).fetch_all(&mut conn).await;
        }

        let end = Instant::now();

        println!(
            "{}: Avg time is {}",
            $n,
            end.duration_since(start).as_millis() / 3u128
        );
    };
}
