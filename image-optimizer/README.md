# Image Optimizer

CloudFront + Lambda@Edge による画像リサイズ・最適化機能。

## 概要

- S3から取得した画像をリサイズしてWebP形式で返す
- CloudFrontでキャッシュ（TTL: 1年）
- クエリパラメータ `w` でサイズ指定（最大1200px）

## 前提条件

- Docker（`--use-container`ビルドに必要）
- AWS CLI & SAM CLI
- AWS SSO ログイン済み

## デプロイ手順

### 1. Image Optimizer（Lambda@Edge）のデプロイ

```bash
cd image-optimizer
make deploy
```

**注意**: Lambda@Edgeは `us-east-1` にデプロイされます。

### 2. Lambda ARNの取得

```bash
aws cloudformation describe-stacks \
  --stack-name image-optimizer \
  --region us-east-1 \
  --query 'Stacks[0].Outputs[?OutputKey==`FunctionArn`].OutputValue' \
  --output text
```

### 3. CloudFrontのデプロイ

```bash
aws cloudformation deploy \
  --template-file cfn/cloudfront.yaml \
  --stack-name cloudfront \
  --parameter-overrides \
    ImageOptimizerLambdaFunctionArn="<上記で取得したARN>" \
    PublicKey="$(cat public_key.pem)" \
  --region ap-northeast-1
```

### 4. 環境変数の設定（SSM）

以下をSSMパラメータに追加:

```
CLOUDFRONT_DOMAIN=<CloudFrontドメイン>
CLOUDFRONT_KEY_PAIR_ID=<公開鍵ID>
CLOUDFRONT_PRIVATE_KEY=<Base64エンコード済み秘密鍵>
```

## 署名用キーペアの作成

```bash
# 秘密鍵を作成（RSA形式）
openssl genrsa -out private_key.pem 2048

# 公開鍵を作成
openssl rsa -pubout -in private_key.pem -out public_key.pem

# 秘密鍵をBase64エンコード（SSM用）
cat private_key.pem | base64 | tr -d '\n'
```

## 更新時の注意

### Lambda@Edgeの更新

Lambda@Edgeを更新するたびに新しいバージョンARNが発行されます。
CloudFrontも新しいARNで更新が必要です。

### CloudFrontリソースの削除

CloudFrontリソースを削除する場合、以下の順序で実行:

1. CloudFront Distribution を削除
2. KeyGroup, PublicKey, CachePolicy, OriginAccessControl を削除
3. Lambda@Edge のレプリカ削除を待つ（数時間かかる場合あり）
4. Lambda スタックを削除

Lambda@Edgeはエッジロケーションにレプリカが作成されるため、
CloudFrontとの関連付けを解除してからでないと削除できません。
