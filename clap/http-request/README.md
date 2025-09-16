# http-request

A simple HTTP client CLI tool built with Rust, Clap, and Reqwest.

## Features

- Custom HTTP methods: GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS
- Add custom headers
- Send raw or JSON request bodies
- Configurable timeout
- Toggle redirect following
- Verbose output with headers and status

## Usage

```sh
cargo run -- [OPTIONS] <URL>
```

### Options

- `-X, --method <METHOD>` HTTP method to use (default: get)
- `-H, --header <HEADER>` Add a header (format: 'Key: Value'). Can be used multiple times.
- `-d, --data <BODY>` Raw request body
- `-j, --json <JSON>` JSON request body (sets Content-Type to application/json)
- `-t, --timeout <SECONDS>` Request timeout in seconds (default: 30)
- `--no-redirect` Don't follow redirects
- `-v, --verbose` Verbose output (prints request and response headers)

## Examples

### Simple GET request

```sh
cargo run -- https://httpbin.org/get
```

### GET request (verbose)

```sh
cargo run -- https://httpbin.org/get -v
```

### POST JSON

```sh
cargo run -- --method post https://httpbin.org/post -j '{"name":"jack","age":20}'
```

### POST with custom header

```sh
cargo run -- -X POST -H 'Authorization: Bearer TOKEN' -d 'some data' https://example.com
```

### Timeout and no redirects

```sh
cargo run -- --timeout 5 --no-redirect https://example.com
```

## License

MIT