# Relational Database (RDB) for Rust

## Diesel setup

```shell
cargo install diesel_cli --no-default-features --features postgres
diesel setup
```

## Diesel migration

```shell
diesel migration generate create_users
diesel migration run
```