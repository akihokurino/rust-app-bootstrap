# Rust App Template For AWS

## Setup

```shell
1. touch .env
2. touch .envrc
```

## Build App

```shell
make build
```

## Run App

```shell
make run-local-db
make run-local-api
```

## Deploy App

```shell
make deploy
```

## Create AWS Resources

```shell
1. make ssm-envs
2. aws cloudformation deploy --template-file cfn/network.yaml --stack-name network
3. aws cloudformation deploy --template-file cfn/rds.yaml --stack-name rds
4. aws cloudformation deploy --template-file cfn/bastion.yaml --stack-name bastion
5. aws cloudformation deploy --template-file cfn/s3.yaml --stack-name s3 --parameter-overrides S3BucketNamePrefix=rust-app-bootstrap
6. aws cloudformation deploy --template-file cfn/sns.yaml --stack-name sns
7. aws cloudformation deploy --template-file cfn/sqs.yaml --stack-name sqs
8. aws cloudformation deploy --template-file cfn/cognito.yaml --stack-name cognito
```

## SeaORM

### Install tools

```shell
cargo install sea-orm-cli
sea-orm-cli migrate init
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