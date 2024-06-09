## setup cargo.toml workspace
```toml
[workspace]

members = [
...
]

resolver = "2"
```

```shell
cargo add axum serde_json tokio --package axum-auth --features tokio/full
```
```shell
cargo add actix-web serde_json --package actix-auth
```

## running from root
```shell
cargo run --bin axum-auth
cargo run --bin actix-auth
```

adding dependencies for jwt-lib --lib
```shell
cargo add chrono jsonwebtoken serde --package jwt-lib --features serde/derive
```