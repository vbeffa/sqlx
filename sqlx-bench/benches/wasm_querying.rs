use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sqlx::Row;
use sqlx::{postgres::PgRow, Connection};
use sqlx::{Database, PgConnection, Postgres};
use sqlx_rt::spawn;

const URL: &str = "postgresql://paul:pass123@127.0.0.1:8080/jetasap_dev";

fn select() {
    spawn(async {
        let mut conn = <Postgres as Database>::Connection::connect(URL)
            .await
            .unwrap();

        let airports = sqlx::query("select * from airports")
            .fetch_all(&mut conn)
            .await;
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| select()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
