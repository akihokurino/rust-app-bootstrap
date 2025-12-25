SAM_CONFIG_FILE := samconfig.dev.toml

.PHONY: build
build:
	rm -rf node_modules package-lock.json .aws-sam
	sam build --use-container

.PHONY: deploy
deploy:
	rm -rf node_modules package-lock.json .aws-sam
	sam build --use-container
	sam deploy \
		--no-confirm-changeset \
		--no-fail-on-empty-changeset \
		--config-file $(SAM_CONFIG_FILE) \
		--parameter-overrides "S3BucketName=matching-app-dev-userdata DeploymentTimestamp=$(shell date +%Y%m%d%H%M%S)"
