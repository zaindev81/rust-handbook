# sqlx-hello

```sh
docker run --name postgres-test \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=test \
  -p 5432:5432 \
  -d postgres:15
```