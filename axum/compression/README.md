# compression

- https://github.com/tokio-rs/axum/tree/main/examples/compression

```sh
# run
cargo run

# test
cargo test

# Sending compressed requests
curl -X POST http://localhost:3000/ \
  -H "Content-Type: application/json" \
  -d '{"message": "Axum test"}'

# Sending uncompressed requests
curl -v -g 'http://localhost:3000/' \
    -H "Content-Type: application/json" \
    --compressed \
    --data-binary @data/products.json
```