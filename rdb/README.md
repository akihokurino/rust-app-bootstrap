# Relational Database (RDB) for Rust

## Diesel CLI

```shell
cargo install diesel_cli --no-default-features --features postgres
```

## Diesel migration

```shell
diesel setup
diesel migration generate create_users
diesel migration run
```