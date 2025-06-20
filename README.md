# Rust App Template For AWS

## Build App

```shell
make build
```

## Run App

```shell
make run-db
make run-local
```

## Deploy App

```shell
make deploy
```

## Create AWS Resources

```shell
1. make ssm
2. aws cloudformation deploy --template-file cfn/network.yaml --stack-name network
3. aws cloudformation deploy --template-file cfn/rds.yaml --stack-name rds
4. aws cloudformation deploy --template-file cfn/bastion.yaml --stack-name bastion
5. aws cloudformation deploy --template-file cfn/s3.yaml --stack-name s3 --parameter-overrides S3BucketNamePrefix=rust-app-bootstrap
6. aws cloudformation deploy --template-file cfn/sns.yaml --stack-name sns
```

## SQLx

### Install tools

```shell
cargo install sqlx-cli --features postgres
```

### Migration

```shell
sqlx migrate add create_users_table -r
sqlx migrate run
```

### Offline mode

```shell
cargo sqlx prepare --workspace
```

### Memo

> IntellijIdea で `set DATABASE_URL to use query macros online` のエラーが出る場合

Settings → Languages & Frameworks → Rust → Environment Variables で下記を追加

```dotenv
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/app
```
