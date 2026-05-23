DOCKER_CMD_BASE :=
DOCKER_EXTRA_PARAMS :=
ifeq ($(USE_DOCKER), 1)
	DOCKER_CACHE_PARAMS :=
	ifeq ($(USE_DOCKER_CACHE), 1)
		DOCKER_CACHE_PARAMS := -v "$(shell pwd)/.docker/cache/cargo/registry:/root/.cargo/registry"
	endif
	DOCKER_CMD_BASE := docker run --rm -v "$(shell pwd):/volume" $(DOCKER_CACHE_PARAMS) $(DOCKER_EXTRA_PARAMS) clux/muslrust:1.92.0-stable
endif

BIN_OUTPUT_DIR := target/x86_64-unknown-linux-musl/release
SRC_FILES := $(shell find . -type f | grep -v '^\./target' | grep -v '^\./\.aws-sam' | grep -v '^\./dist' | grep -v '/\.')
DEPLOY_CRATES := api async_sns_fn sync_fn batch_fn async_sqs_fn
DIST_DIR := dist
COGNITO_USER_POOL_ID :=ap-northeast-1_qyBWnc7Q7
COGNITO_USER_NAME := admin-owner
AWS_SSO_SESSION ?= dev

$(BIN_OUTPUT_DIR)/%: $(SRC_FILES)
	$(DOCKER_CMD_BASE) cargo build --release --bin $(lastword $(subst /, ,$@)) --target x86_64-unknown-linux-musl
	if [ "$(STRIP)" = "1" ]; then strip $@; fi

# Each build-<Function> runs inside .aws-sam/build/<Function>/, where CodeUri (dist/<func>/)
# has been copied. The artifact is already there as bootstrap - just cp it.
build-ApiFunction:
	cp bootstrap $(ARTIFACTS_DIR)/bootstrap

build-AsyncSnsFunction:
	cp bootstrap $(ARTIFACTS_DIR)/bootstrap

build-AsyncSqsFunction:
	cp bootstrap $(ARTIFACTS_DIR)/bootstrap

build-SyncFunction:
	cp bootstrap $(ARTIFACTS_DIR)/bootstrap

build-BatchFunction:
	cp bootstrap $(ARTIFACTS_DIR)/bootstrap

.PHONY: build
build: $(addprefix $(BIN_OUTPUT_DIR)/,$(DEPLOY_CRATES))

# Stage prebuilt artifacts into dist/<func>/ so each CodeUri is tiny.
# Each dist/<func>/ also gets a minimal Makefile so SAM CustomMakeBuilder works.
.PHONY: dist
dist: build
	rm -rf $(DIST_DIR)
	$(foreach crate,$(DEPLOY_CRATES),mkdir -p $(DIST_DIR)/$(crate) && cp $(BIN_OUTPUT_DIR)/$(crate) $(DIST_DIR)/$(crate)/bootstrap && cp dist.Makefile.rust $(DIST_DIR)/$(crate)/Makefile;)

.PHONY: deploy
deploy: dist
	sam build
	sam deploy --no-confirm-changeset --no-fail-on-empty-changeset

.PHONY: run-local-api
run-local-api:
	SSM_DOTENV_PARAMETER_NAME=/app/server/dotenv IS_LOCAL=true cargo run --bin api

.PHONY: run-local-db
run-local-db:
	docker-compose up db

.PHONY: reset-local-db
reset-local-db:
	docker-compose down -v && docker-compose up

.PHONY: run-migration
run-migration:
	cd migration && cargo run

.PHONY: gen
gen:
	sea-orm-cli generate entity \
	-u $(if $(DATABASE_URL),$(DATABASE_URL),postgresql://postgres:postgres@localhost:5432/app) \
	-o app/src/infra/rdb/generated
	{ echo '#![allow(unused)]'; cat app/src/infra/rdb/generated/prelude.rs; } > app/src/infra/rdb/generated/prelude.rs.tmp && \
    mv app/src/infra/rdb/generated/prelude.rs.tmp app/src/infra/rdb/generated/prelude.rs

.PHONY: gen-migration-file
gen-migration-file:
	@read -p "Enter migration name: " name; \
	sea-orm-cli migrate generate $$name

.PHONY: connect-rds
connect-rds:
	@INSTANCE_ID=$$(aws ec2 describe-instances --filters "Name=tag:Name,Values=bastion" "Name=instance-state-name,Values=running" --query 'Reservations[0].Instances[0].InstanceId' --output text) && \
	RDS_ENDPOINT=$$(aws cloudformation describe-stacks --stack-name rds --query 'Stacks[0].Outputs[?OutputKey==`DatabaseEndpoint`].OutputValue' --output text) && \
	echo "pgAdmin4で localhost:5432 に接続してください" && \
	echo "ユーザー: postgres, パスワード: postgres, DB: app" && \
	aws ssm start-session --target $${INSTANCE_ID} --document-name AWS-StartPortForwardingSessionToRemoteHost --parameters "{\"host\":[\"$${RDS_ENDPOINT}\"],\"portNumber\":[\"5432\"],\"localPortNumber\":[\"5432\"]}"

.PHONY: connect-bastion
connect-bastion:
	@INSTANCE_ID=$$(aws ec2 describe-instances --filters "Name=tag:Name,Values=bastion" "Name=instance-state-name,Values=running" --query 'Reservations[0].Instances[0].InstanceId' --output text) && \
	echo "Bastionホストに接続中..." && \
	aws ssm start-session --target $${INSTANCE_ID}

# SecureStringをCloudFormation経由で作成できない
.PHONY: ssm-envs
ssm-envs:
	aws ssm put-parameter \
	--name "/app/server/dotenv" \
	--value "ENV=dev" \
	--type "SecureString"

.PHONY: ssm-docker-config
ssm-docker-config:
	aws ssm put-parameter \
	--name "/app/docker/config" \
	--value file://.docker/config.json \
	--type "SecureString"

.PHONY: create-admin-user
create-admin-user:
	aws cognito-idp admin-create-user \
		--user-pool-id $(COGNITO_USER_POOL_ID) \
		--username $(COGNITO_USER_NAME) \
		--temporary-password Test1234 \
		--message-action SUPPRESS \
		--user-attributes Name=email,Value=aki030402@gmail.com Name=email_verified,Value=true
	aws cognito-idp admin-set-user-password \
		--user-pool-id $(COGNITO_USER_POOL_ID) \
		--username $(COGNITO_USER_NAME) \
		--password Test1234 \
		--permanent

.PHONY: login-admin-user
login-admin-user:
	$(eval CLIENT_ID := $(shell aws cognito-idp list-user-pool-clients --user-pool-id $(COGNITO_USER_POOL_ID) --query 'UserPoolClients[0].ClientId' --output text))
	aws cognito-idp admin-initiate-auth \
		--user-pool-id $(COGNITO_USER_POOL_ID) \
		--client-id $(CLIENT_ID) \
		--auth-flow ADMIN_USER_PASSWORD_AUTH \
		--auth-parameters USERNAME=$(COGNITO_USER_NAME),PASSWORD=Test1234

.PHONY: login-sso
login-sso:
	aws sso login --sso-session $(AWS_SSO_SESSION)