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

### IntellijIdea で `set DATABASE_URL to use query macros online` のエラーが出る場合

Settings → Languages & Frameworks → Rust → Environment Variables

```dotenv
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/app
```

を追加。