#![feature(test)]

extern crate test;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use sqlx::Connection;
use sqlx::{Database, PgConnection, Postgres};

pub const URL: &str = "postgresql://paul:pass123@127.0.0.1:8080/jetasap_dev";

pub async fn new() -> PgConnection {
    <Postgres as Database>::Connection::connect(URL)
        .await
        .unwrap()
}

#[macro_export]
macro_rules! time_query {
    ($n:expr, $q:expr) => {
        let mut conn = sqlx_wasm_test::new().await;

        let performance = web_sys::window().unwrap().performance().unwrap();
        let start = performance.now();

        for _ in 0..3u8 {
            let _ = sqlx::query($q).fetch_all(&mut conn).await;
        }

        let end = performance.now();
        web_sys::console::log_1(&format!("{}: Avg time is {}", $n, (end - start) / 3f64).into());
    };
}

#[macro_export]
macro_rules! time_insert_query {
    ($n:expr, $count:literal) => {
        let mut conn = sqlx_wasm_test::new().await;
        let _ = conn
            .execute("create temp table bench_inserts (id integer, descr text)")
            .await;

        let performance = web_sys::window().unwrap().performance().unwrap();
        let start = performance.now();

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

        let end = performance.now();
        web_sys::console::log_1(&format!("{}: Avg time is {}", $n, (end - start) / 3f64).into());
    };
}

#[macro_export]
macro_rules! time_update_query {
    ($n:expr, $count:literal) => {
        let mut conn = sqlx_wasm_test::new().await;
        let _ = conn.execute("create temp table bench_updates (id integer, descr text, primary key(id))")
            .await;
        let _ = conn.execute("create bitmap index id_idx on bench_updates (id)")
            .await;

        let _ = sqlx::query(&format!(
            "insert into bench_updates (id, descr) select generate_series(1,{}) AS id, md5(random()::text) AS descr",
            $count
        ))
        .execute(&mut conn)
        .await;

        let performance = web_sys::window().unwrap().performance().unwrap();
        let start = performance.now();

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

        let end = performance.now();
        web_sys::console::log_1(&format!("{}: Avg time is {}", $n, (end - start) / 3f64).into());
    };
}

#[macro_export]
macro_rules! time_delete_query {
    ($n:expr, $count:literal) => {
        let mut conn = sqlx_wasm_test::new().await;
        let _ = conn.execute("create temp table bench_deletes (id integer, descr text, primary key(id))")
            .await;

        let _ = conn.execute("create bitmap index id_idx on bench_deletes (id)")
            .await;

        let _ = sqlx::query(&format!(
            "insert into bench_deletes (id, descr) select generate_series(1,{}) AS id, md5(random()::text) AS descr",
            $count
        ))

        .execute(&mut conn)
        .await;
        let performance = web_sys::window().unwrap().performance().unwrap();
        let start = performance.now();

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

        let end = performance.now();
        web_sys::console::log_1(&format!("{}: Avg time is {}", $n, (end - start) / 3f64).into());
    };
}

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

#[macro_export]
macro_rules! test_unprepared_type {
    ($name:ident<$ty:ty>($db:ident, $($text:literal == $value:expr),+ $(,)?)) => {
        paste::item! {
            #[wasm_bindgen_test::wasm_bindgen_test]
            async fn [< test_unprepared_type_ $name >] () {
                use sqlx::prelude::*;
                use futures::TryStreamExt;

                let mut conn = sqlx_wasm_test::new().await;

                $(
                    let query = format!("SELECT {}", $text);
                    let mut s = conn.fetch(&*query);
                    let row = s.try_next().await.unwrap().unwrap();
                    let rec = row.try_get::<$ty, _>(0).unwrap();

                    assert!($value == rec);

                    drop(s);
                )+
            }
        }
    }
}

#[macro_export]
macro_rules! __test_prepared_type {
    ($name:ident<$ty:ty>($db:ident, $sql:expr, $($text:literal == $value:expr),+ $(,)?)) => {
        paste::item! {
            #[wasm_bindgen_test::wasm_bindgen_test]
            async fn [< test_prepared_type_ $name >] () {
                use sqlx::Row;

                let mut conn = sqlx_wasm_test::new().await;

                $(
                    let query = format!($sql, $text);

                    let row = sqlx::query(&query)
                        .bind($value)
                        .bind($value)
                        .fetch_one(&mut conn)
                        .await.unwrap();

                    let matches: i32 = row.try_get(0).unwrap();
                    let returned: $ty = row.try_get(1).unwrap();
                    let round_trip: $ty = row.try_get(2).unwrap();

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
            }
        }
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

// Test type decoding only for the prepared query API
#[macro_export]
macro_rules! __test_prepared_decode_type {
    ($name:ident<$ty:ty>($db:ident, $($text:literal == $value:expr),+ $(,)?)) => {
        paste::item! {
            #[wasm_bindgen_test::wasm_bindgen_test]
            async fn [< test_prepared_decode_type_ $name >] () {
                use sqlx::Row;

                let mut conn = sqlx_wasm_test::new().await;

                $(
                    let query = format!("SELECT {}", $text);

                    let row = sqlx::query(&query)
                        .fetch_one(&mut conn)
                        .await.unwrap();

                    let rec: $ty = row.try_get(0).unwrap();

                    assert!($value == rec);
                )+
            }
        }
    };
}

#[macro_export]
macro_rules! Postgres_query_for_test_prepared_type {
    () => {
        "SELECT ({0} is not distinct from $1)::int4, {0}, $2"
    };
}
