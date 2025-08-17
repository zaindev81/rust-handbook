# template-server

```sh
# run
cargo run .

# http
http http://localhost:3000
http http://localhost:3000/health
http http://localhost:3000/counter
http POST http://localhost:3000/counter/increment
http POST http://localhost:3000/counter/set/2000000
```