# ---- Project metadata ----
PACKAGE_NAME := intervaltree_rs
PYTHON       ?= python3
BUILD_DIR    := target/wheels

# ---- Commands ----

.PHONY: help build develop publish clean

help:
	@echo "Available commands:"
	@echo "  make build     - Build release wheels (and sdist)"
	@echo "  make develop   - Install locally in dev mode"
	@echo "  make publish   - Publish to PyPI using maturin"
	@echo "  make clean     - Remove build artifacts"

build:
	@echo "🚀 Building release wheels..."
	maturin build --release --sdist
	@echo "✅ Wheels generated in $(BUILD_DIR)/"

develop:
	@echo "💡 Installing in development mode..."
	maturin develop

publish: build
	@echo "📦 Publishing $(PACKAGE_NAME) to PyPI..."
	@if [ -z "$$MATURIN_PASSWORD" ]; then \
		echo "❌ Error: MATURIN_PASSWORD (your PyPI token) is not set."; \
		echo "   export MATURIN_USERNAME=__token__"; \
		echo "   export MATURIN_PASSWORD=pypi-xxxxxx"; \
		exit 1; \
	fi
	maturin publish --skip-existing
	@echo "✅ Published successfully!"

clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	rm -rf $(BUILD_DIR) dist build *.egg-info
