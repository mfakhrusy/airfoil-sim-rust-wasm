.PHONY: help build start serve clean test install-deps

# Default target
help:
	@echo "🛩️  Airfoil Simulation WASM - Available Commands:"
	@echo ""
	@echo "  make build       - Build the Rust WASM package"
	@echo "  make start       - Start the development server (port 8000)"
	@echo "  make serve       - Alias for start"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make test        - Run Rust tests"
	@echo "  make test-web    - Run web tests"
	@echo "  make install-deps - Install required dependencies"
	@echo "  make help        - Show this help message"
	@echo ""

# Build the WASM package
build:
	@echo "🦀 Building Rust WASM package..."
	wasm-pack build --target web
	@echo "✅ Build complete! Files generated in pkg/"

# Start development server
start:
	@echo "🌐 Starting development server on port 8000..."
	@echo "📂 Serving files from: $$(pwd)"
	@echo "🔗 Open http://localhost:8000 in your browser"
	@echo "⏹️  Press Ctrl+C to stop the server"
	python3 -m http.server 8000

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	rm -rf pkg/
	rm -rf target/
	@echo "✅ Clean complete!"

# Run Rust tests
test:
	@echo "🧪 Running Rust tests..."
	cargo test

# Run web tests (requires wasm-pack)
test-web:
	@echo "🌐 Running web tests..."
	wasm-pack test --headless --firefox

# Install required dependencies
install-deps:
	@echo "📦 Installing required dependencies..."
	@echo "Checking for cargo..."
	@command -v cargo >/dev/null 2>&1 || { \
		echo "❌ cargo is required but not installed."; \
		echo "Please install Rust and cargo from https://rustup.rs/"; \
		exit 1; \
	}
	@echo "Checking for wasm-pack..."
	@command -v wasm-pack >/dev/null 2>&1 || { \
		echo "Installing wasm-pack..."; \
		cargo install wasm-pack; \
	}
	@echo "Checking for Python..."
	@command -v python3 >/dev/null 2>&1 || { \
		echo "❌ Python3 is required but not installed."; \
		echo "Please install Python3 to use the development server."; \
		exit 1; \
	}
	@echo "✅ All dependencies are available!"

# Development workflow - build and serve
dev: build start

# Production build with optimizations
build-release:
	@echo "🚀 Building optimized release version..."
	wasm-pack build --target web --release
	@echo "✅ Release build complete!"

# Check project status
status:
	@echo "📊 Project Status:"
	@echo "- Rust version: $$(rustc --version 2>/dev/null || echo 'Not installed')"
	@echo "- wasm-pack: $$(wasm-pack --version 2>/dev/null || echo 'Not installed')"
	@echo "- Python: $$(python3 --version 2>/dev/null || echo 'Not installed')"
	@echo "- Build artifacts:"
	@if [ -d "pkg" ]; then \
		echo "  ✅ pkg/ directory exists"; \
		ls -la pkg/ | head -5; \
	else \
		echo "  ❌ pkg/ directory not found (run 'make build')"; \
	fi

# Quick setup for new users
setup: install-deps build
	@echo ""
	@echo "🎉 Setup complete! Run 'make start' to launch the application."
