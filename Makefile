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
DEPLOY_CRATES := api

$(BIN_OUTPUT_DIR)/%: $(SRC_FILES)
	$(DOCKER_CMD_BASE) cargo build --release --bin $(lastword $(subst /, ,$@)) --target x86_64-unknown-linux-musl
	if [ "$(STRIP)" = "1" ]; then strip $@; fi

build-ApiFunction: $(BIN_OUTPUT_DIR)/api
	cp $< $(ARTIFACTS_DIR)/bootstrap

.PHONY: build
build: $(addprefix $(BIN_OUTPUT_DIR)/,$(DEPLOY_CRATES))

.PHONY: ssm
ssm:
	aws ssm put-parameter \
	--name "/rust-app-bootstrap/server/dotenv" \
	--value "ENV=dev" \
	--type "SecureString" \
 	--overwrite

.PHONY: deploy
deploy: $(addprefix $(BIN_OUTPUT_DIR)/,$(DEPLOY_CRATES))
	sam build
	sam deploy --no-confirm-changeset --no-fail-on-empty-changeset

.PHONY: run-local
run-local:
	SSM_DOTENV_PARAMETER_NAME=/rust-app-bootstrap/server/dotenv cargo run --bin api