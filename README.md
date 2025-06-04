# Rust App Template

## Build App

```shell
cargo build
```

## Run App

```shell
cargo run
```

## SQLx

### install tools

```shell
cargo install sqlx-cli --features postgres
```

### migration

```shell
sqlx migrate add create_users_table -r
sqlx migrate run
```