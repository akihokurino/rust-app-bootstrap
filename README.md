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
5. aws cloudformation deploy --template-file cfn/s3.yaml --stack-name s3
6. aws cloudformation deploy --template-file cfn/sns.yaml --stack-name sns
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
