# Setup
1. Make sure postgres is installed and listening at port 5432.
2. Start a websocket-tcp proxy using [websocat](https://github.com/vi/websocat)
   `$ websocat --binary ws-l:127.0.0.1:8080 tcp:127.0.0.1:5432`

# Running
From the root folder of this crate:
1. `wasm-pack test --firefox -- --test`
2. Launch Firefox and navigate to [http://127.0.0.1:8000](http://127.0.0.1:8000)

Corresponding native queries' benchmarking is done in `../tests/postgres/*_custom_bench.rs` files and they can be run by executing (insert benchmarking for example)  -
`$ cargo test --no-default-features --features postgres,runtime-async-std-rustls --test pg-inserts-bench -- --test-threads=1 --nocapture`
from the root of this repo.
