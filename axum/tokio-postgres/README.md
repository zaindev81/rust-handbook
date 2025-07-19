# tokio-postgres

## setup

```sh
rustup install nightly
rustup override set nightly

# docker
docker run --rm --name postgres-test \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=test \
  -p 5432:5432 \
  -d postgres:16
```

## development

```sh
cargo run

# http
http http://localhost:3000
http http://localhost:3000/health
http http://localhost:3000/db-health
```