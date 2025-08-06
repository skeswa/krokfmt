# krokfmt Deployment

## Documentation Site

The krokfmt documentation and playground is deployed as a static site to GitHub Pages.

### Architecture

```
GitHub Pages (Static Hosting)
    ├── VitePress Documentation (HTML/CSS/JS)
    ├── Interactive Playground (Vue.js)
    └── WASM Module (krokfmt compiled to WebAssembly)
```

### Automatic Deployment

The site automatically deploys to GitHub Pages when changes are pushed to the main branch:

1. **Trigger**: Push to `main` branch or manual workflow dispatch
2. **Build**: 
   - Compile krokfmt to WASM using wasm-pack
   - Build VitePress documentation site
   - Bundle everything as static files
3. **Deploy**: Upload to GitHub Pages

The site will be available at: `https://[username].github.io/krokfmt/`

### Local Development

```bash
# Build and run locally
cargo xtask run-web
# Or use the shortcut:
cargo web

# Visit http://localhost:3000
```

### Manual Build

```bash
# Build WASM module
cargo xtask build-wasm --release

# Build documentation site
cd crates/krokfmt-web
npm install
npm run build

# The built site is in docs/.vitepress/dist/
# Upload this directory to any static hosting service
```

### Hosting Options

Since it's a pure static site with client-side WASM, it can be hosted anywhere:

- **GitHub Pages** (current) - Free, automatic deployments
- **Netlify** - Free tier, automatic deploys from Git
- **Vercel** - Free tier, great performance
- **Cloudflare Pages** - Free tier, global CDN
- **AWS S3 + CloudFront** - Scalable, pay-per-use
- **Any static file server** - nginx, Apache, etc.

## Docker Deployment (Optional)

For self-hosting or development, a Docker image is available:

### Build and Run

```bash
# Build the Docker image
docker build -f deployment/docker/Dockerfile.web -t krokfmt-web .

# Run the container
docker run -p 8080:80 krokfmt-web

# Access at http://localhost:8080
```

### Docker Compose

```yaml
version: '3.8'

services:
  krokfmt-web:
    build:
      context: .
      dockerfile: deployment/docker/Dockerfile.web
    ports:
      - "8080:80"
    restart: unless-stopped
```

### Pre-built Images

Pre-built Docker images are available from GitHub Container Registry:

```bash
docker run -p 8080:80 ghcr.io/skeswa/krokfmt-web:latest
```

## Key Benefits

- **No Server Required** - Pure static files
- **Client-Side Formatting** - WASM runs in the browser
- **Privacy** - Code never leaves the user's machine
- **Offline Capable** - Works without internet after initial load
- **Infinitely Scalable** - CDN handles all traffic
- **Zero Server Costs** - Use free static hosting

## Environment Variables

No environment variables are required since everything runs client-side.

## Monitoring

Since it's a static site, monitoring is simple:

- **Uptime**: GitHub Pages status or your CDN's monitoring
- **Analytics**: Google Analytics, Plausible, or similar (optional)
- **Errors**: Browser console errors (no server logs)

## Security

The static site architecture is inherently secure:

- **No Server Vulnerabilities** - No backend to attack
- **No Data Storage** - All processing happens client-side
- **No API Keys** - No secrets needed
- **HTTPS by Default** - GitHub Pages provides SSL