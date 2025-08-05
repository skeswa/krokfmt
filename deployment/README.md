# krokfmt Web Deployment

This directory contains deployment configurations for the krokfmt web interface.

## Quick Start

### Local Development

```bash
# Install dependencies (one-time)
cargo xtask install-deps

# Build and run locally
cargo xtask run-web
# Or use the shortcut:
cargo web

# Visit http://localhost:3000
```

### Docker Deployment

```bash
# Build Docker image
cargo xtask docker-build

# Run Docker container
cargo xtask docker-run
# Visit http://localhost:3000
```

### Kubernetes Deployment (Rhuidean)

```bash
# Build and push Docker image
docker build -f deployment/docker/Dockerfile.web -t ghcr.io/skeswa/krokfmt-web:latest .
docker push ghcr.io/skeswa/krokfmt-web:latest

# Deploy to Kubernetes
kubectl apply -f deployment/k8s/
```

## Architecture

The web deployment consists of:

1. **Web Server** (`krokfmt-web`): Axum-based Rust web server
   - Serves static files and templates
   - Hosts WASM modules for browser execution
   - Provides fallback API endpoint for formatting

2. **WASM Module** (`krokfmt-playground`): Client-side formatter
   - Runs entirely in the browser
   - No server round-trips for formatting
   - Built with wasm-pack and wasm-bindgen

3. **Static Assets**: CSS, JavaScript, and HTML templates
   - Modern responsive design
   - Interactive playground interface
   - Documentation pages

## Build Process

The build process is integrated:

1. **WASM Build**: Compiles Rust to WebAssembly
   - Uses `wasm-pack` to generate JS bindings
   - Optimized for size with `wee_alloc`
   - Development builds skip wasm-opt due to compatibility

2. **Web Server Build**: Standard Rust compilation
   - Includes all static assets
   - Serves WASM modules at `/wasm/*`
   - API fallback at `/api/format`

3. **Docker Build**: Multi-stage build
   - Stage 1: Build WASM and server binary
   - Stage 2: Minimal runtime image with assets

## Endpoints

- `/` - Home page with feature overview
- `/docs` - Documentation
- `/playground` - Interactive formatter (WASM-powered)
- `/api/format` - Server-side formatting API (fallback)
- `/health` - Health check endpoint
- `/static/*` - Static assets (CSS, JS)
- `/wasm/*` - WASM modules and bindings

## Environment Variables

- `RUST_LOG` - Log level (default: `krokfmt_web=info,tower_http=debug`)
- Port is hardcoded to 3000 (can be changed in source)

## Notes

- WASM formatting is preferred for better performance
- Server API exists as fallback for WASM-incompatible browsers
- The playground automatically detects JSX and uses appropriate parsing