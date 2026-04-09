.PHONY: help build test lint format clean install all

help:
	@echo "OILSHIP build system"
	@echo ""
	@echo "  make build     build all components (anchor + watch + sdk + cli)"
	@echo "  make test      run all tests"
	@echo "  make lint      run clippy + eslint + ruff"
	@echo "  make format    rustfmt + prettier + ruff format"
	@echo "  make install   install local toolchain dependencies"
	@echo "  make clean     remove build artifacts"
	@echo "  make all       install + build + test + lint"

install:
	@cd sdk && npm install
	@cd cli && pip install -e .

build:
	@anchor build || cargo build --workspace --release
	@cd sdk && npm run build

test:
	@cargo test --workspace
	@cd sdk && npm test --if-present
	@cd cli && pytest -q || true

lint:
	@cargo clippy --workspace --all-targets -- -D warnings
	@cd sdk && npm run lint --if-present
	@cd cli && ruff check . || true

format:
	@cargo fmt --all
	@cd sdk && npm run format --if-present
	@cd cli && ruff format . || true

clean:
	@cargo clean
	@rm -rf sdk/dist sdk/node_modules
	@find . -name "__pycache__" -type d -exec rm -rf {} + 2>/dev/null || true

all: install build test lint
