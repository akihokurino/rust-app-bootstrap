# Rust App Template For AWS

## Build App

```shell
make build
```

## Run App

```shell
make run-db
make run-api
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

## SeaORM

### Install tools

```shell
cargo install sea-orm-cli
sea-orm-cli migrate init
```

### Migration

```shell
sea-orm-cli migrate generate create_table_users
cargo run -- refresh
```

## Docker auth

```shell
echo -n "username:personal_access_token" | base64
```

```json
{
  "auths": {
    "https://index.docker.io/v1/": {
      "auth": "<base64-encoded-auth>"
    }
  }
}
```

## AWS

<img width="1220" alt="スクリーンショット 2025-06-27 10 44 46" src="https://github.com/user-attachments/assets/ea65fafc-067f-4923-ba4e-27dd80bcbd15" />
