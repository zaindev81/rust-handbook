# redis

```sh
# docker
docker run --rm --name redis-test \
  -p 6379:6379 \
  -e REDIS_PASSWORD=yourpassword \
  -d redis:7 \
  redis-server --requirepass yourpassword
```

# Run

```sh
cargo run --example bb8_quickstart
```