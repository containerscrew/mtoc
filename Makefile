SHELL:=/bin/sh
.PHONY: all

BINARY_NAME = mtoc


help: ## this help
	@awk 'BEGIN {FS = ":.*?## ";  printf "Usage:\n  make \033[36m<target> \033[0m\n\nTargets:\n"} /^[a-zA-Z0-9_-]+:.*?## / {gsub("\\\\n",sprintf("\n%22c",""), $$2);printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

pre-commit: ## Run pre-commit
	pre-commit run -a

package: ## Package binary with zip
	zip -j ${BINARY_NAME}-$(PLATFORM).zip target/$(TARGET)/release/${BINARY_NAME}

generate-changelog: ## Generate changelog using git cliff
	git cliff --output CHANGELOG.md

build: ## Build binary
	cargo build --release

install: ## Install binary
	cargo install --path .

uninstall: ## Uninstall binary
	cargo uninstall ${BINARY_NAME}

build: ## Build binary
	cargo build --release --locked

cargo-fix: ## Run cargo fix
	cargo fix --bin mtoc

mtoc: ## Create table of contents with doctoc
	mtoc -d .

cargo-login: ## Login to cargo
	cargo login

cargo-publish-dr: ## Publish to cargo (dry-run)
	cargo publish --dry-run

cargo-publish: ## Publish to cargo
	cargo publish
