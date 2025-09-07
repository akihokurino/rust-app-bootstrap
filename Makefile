DOCKER_CMD_BASE :=
DOCKER_EXTRA_PARAMS :=
ifeq ($(USE_DOCKER), 1)
	DOCKER_CACHE_PARAMS :=
	ifeq ($(USE_DOCKER_CACHE), 1)
		DOCKER_CACHE_PARAMS := -v "$(shell pwd)/.docker/cache/cargo/registry:/root/.cargo/registry"
	endif
	DOCKER_CMD_BASE := docker run --rm -v "$(shell pwd):/volume" $(DOCKER_CACHE_PARAMS) $(DOCKER_EXTRA_PARAMS) clux/muslrust:1.87.0-stable
endif


BIN_OUTPUT_DIR := target/x86_64-unknown-linux-musl/release
SRC_FILES := $(shell find . -type f | grep -v '^\./target' | grep -v '/\.')
DEPLOY_CRATES := api async_task sync_task batch_task

$(BIN_OUTPUT_DIR)/%: $(SRC_FILES)
	$(DOCKER_CMD_BASE) cargo build --release --bin $(lastword $(subst /, ,$@)) --target x86_64-unknown-linux-musl
	if [ "$(STRIP)" = "1" ]; then strip $@; fi

build-ApiFunction: $(BIN_OUTPUT_DIR)/api
	cp $< $(ARTIFACTS_DIR)/bootstrap

build-AsyncTaskFunction: $(BIN_OUTPUT_DIR)/async_task
	cp $< $(ARTIFACTS_DIR)/bootstrap

build-SyncTaskFunction: $(BIN_OUTPUT_DIR)/sync_task
	cp $< $(ARTIFACTS_DIR)/bootstrap

build-BatchTaskFunction: $(BIN_OUTPUT_DIR)/batch_task
	cp $< $(ARTIFACTS_DIR)/bootstrap

.PHONY: build
build: $(addprefix $(BIN_OUTPUT_DIR)/,$(DEPLOY_CRATES))

.PHONY: deploy
deploy: $(addprefix $(BIN_OUTPUT_DIR)/,$(DEPLOY_CRATES))
	sam build
	sam deploy --no-confirm-changeset --no-fail-on-empty-changeset

.PHONY: run-api
run-api:
	SSM_DOTENV_PARAMETER_NAME=/app/server/dotenv cargo run --bin api

.PHONY: run-db
run-db:
	docker-compose up db

.PHONY: run-migration
run-migration:
	cd migration && cargo run -- refresh

.PHONY: reset-db
reset-db:
	docker-compose down -v && docker-compose up

.PHONY: connect-rds
connect-rds:
	@INSTANCE_ID=$$(aws ec2 describe-instances --filters "Name=tag:Name,Values=bastion" "Name=instance-state-name,Values=running" --query 'Reservations[0].Instances[0].InstanceId' --output text) && \
	BASTION_IP=$$(aws ec2 describe-instances --instance-ids $${INSTANCE_ID} --query 'Reservations[0].Instances[0].PublicIpAddress' --output text) && \
	RDS_ENDPOINT=$$(aws cloudformation describe-stacks --stack-name rds --query 'Stacks[0].Outputs[?OutputKey==`DatabaseEndpoint`].OutputValue' --output text) && \
	echo "公開鍵を送信中（60秒間有効）..." && \
	aws ec2-instance-connect send-ssh-public-key --instance-id $${INSTANCE_ID} --instance-os-user ec2-user --ssh-public-key "$$(cat ~/.ssh/id_rsa.pub)" && \
	echo "pgAdmin4で localhost:5432 に接続してください" && \
	echo "ユーザー: postgres, パスワード: postgres, DB: app" && \
	ssh -o StrictHostKeyChecking=no -L 5432:$${RDS_ENDPOINT}:5432 ec2-user@$${BASTION_IP}

.PHONY: connect-bastion
connect-bastion:
	@INSTANCE_ID=$$(aws ec2 describe-instances --filters "Name=tag:Name,Values=bastion" "Name=instance-state-name,Values=running" --query 'Reservations[0].Instances[0].InstanceId' --output text) && \
	echo "Bastionホストに接続中..." && \
	aws ec2-instance-connect ssh --instance-id $${INSTANCE_ID} --os-user ec2-user

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