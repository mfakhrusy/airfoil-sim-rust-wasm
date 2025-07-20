# Airfoil Simulation - Hello World Web Frontend

This is a simple web frontend that demonstrates how to use Rust WASM in a web browser.

## 🚀 Quick Start

1. **Build the WASM package**:
   ```bash
   make build
   ```

2. **Start the development server**:
   ```bash
   make start
   ```

3. **Open your browser** to `http://localhost:8000`

## 🛠 Available Make Commands

- `make help` - Show all available commands
- `make build` - Build the Rust WASM package
- `make start` - Start development server (port 8000)
- `make clean` - Clean build artifacts
- `make test` - Run Rust tests
- `make test-web` - Run web tests
- `make status` - Check project status
- `make setup` - Install dependencies and build (for new users)

## 📁 Project Structure

- `index.html` - Main web page with the Hello World demo
- `Makefile` - Build and development commands
- `pkg/` - Generated WASM bindings and files
- `src/lib.rs` - Rust source code with WASM exports

## ✨ What It Does

The web application demonstrates:

- ✅ Loading and initializing a Rust WASM module
- 🦀 Calling Rust functions from JavaScript
- 📝 Rust logging to browser console
- 🔢 Simple function calls with parameters and return values
- 🚨 Alert dialogs from Rust code

## 🛠 Development

To modify the Rust code:

1. Edit files in `src/`
2. Run `make build` to rebuild
3. Refresh your browser (server keeps running)

For a complete development workflow:
```bash
make clean  # Clean previous builds
make build  # Build WASM package  
make start  # Start development server
```

## 🌐 Browser Requirements

- Modern browser with WASM support (Chrome, Firefox, Safari, Edge)
- Must be served over HTTP (not file://) due to WASM security requirements

## 📋 Available Functions

- `greet()` - Shows an alert dialog
- `greet_console()` - Logs messages to browser console
- `get_greeting()` - Returns a greeting string
- `add(a, b)` - Adds two numbers and logs the result

Have fun exploring Rust + WASM! 🦀✨
