# ============================================================
# 	DBSight Workspace Makefile
# ============================================================

# -------- Project Settings --------
CARGO := cargo
TARGET_DIR := target
BUILD_FLAGS :=
RUSTFLAGS ?=

# Default Goal
.DEFAULT_GOAL := help

# -------- Environment --------
export RUST_BACKTRACE=1
export RUST_LOG=info

# ============================================================
# 	Basic Commands
# ============================================================

.PHONY: all build run test fmt lint clean doc release watch help

## Build Debug
build:
	$(CARGO) build $(BUILD_FLAGS)

## Build Release
release:
	$(CARGO) build --release $(BUILD_FLAGS)

## Hot Reload (Requires cargo-watch installation) For the development phase
dev:
	cargo watch -w crates -w apps -x 'run'

## Clean build artifacts
clean:
	$(CARGO) clean

# ============================================================
# 	Test & Lint
# ============================================================

## Run Test
test:
	$(CARGO) test --workspace --all-targets

## Run check
check:
	$(CARGO) check --workspace

## Lint Code
fmt:
	$(CARGO) fmt --all

## Lint Check
lint:
	$(CARGO) clippy --workspace --all-targets --all-features -- -D warnings

# ============================================================
# 	Docs & Update
# ============================================================

## Generate Docs
doc:
	$(CARGO) doc --workspace --no-deps --document-private-items

## Third-party Crates
update:
	$(CARGO) update

# ============================================================
# 	Help Info
# ============================================================

help:
	@echo ""
	@echo "=============================================="
	@echo "  🚀 DBSight Workspace Makefile Commands"
	@echo "=============================================="
	@echo ""
	@awk "/^##/ { \
		line = substr(\$$0, 3); \
		sub(/^[[:space:]]+/, \"\", line); \
		if (desc == \"\") desc = line; \
		else desc = desc \" \" line; \
		next; \
	} \
	/^[a-zA-Z0-9._-]+[[:space:]]*:/ { \
		target = \$$1; \
		sub(/:$$/, \"\", target); \
		sub(/[[:space:]]+$$/, \"\", target); \
		if (desc != \"\") { \
			printf \"  \033[36m%-15s\033[0m %s\n\", target, desc; \
			desc = \"\"; \
		} \
	}" $(MAKEFILE_LIST)
	@echo ""
