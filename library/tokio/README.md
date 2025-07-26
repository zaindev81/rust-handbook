# tokio

- https://tokio.rs/
- https://github.com/tokio-rs/tokio
- https://github.com/tokio-rs/tokio/tree/master/examples

```sh
# hello_world
ncat -l 6142
cargo run --example hello_world

# mini_redis_tutorial
mini-redis-server --port 6380
mini-redis-cli --port 6380 set foo bar
mini-redis-cli --port 6380 get foo

cargo run --example mini_redis_tutorial

# echo_tcp_server
cargo run --example echo_tcp_server
```