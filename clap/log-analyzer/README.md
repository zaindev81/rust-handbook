# log-analyzer

```sh
cargo run -- app.log

cargo run -- app.log error.log access.log

cargo run -- app.log --level error warn

cargo run -- app.log --from 2024-01-01 --to 2024-01-31

cargo run -- app.log --level error warn --from 2024-01-01 --to 2024-01-31 --stats

cargo run -- app.log --output-format json

cargo run -- app.log --pattern "database" --level error

cargo run -- app.log --limit 100
```