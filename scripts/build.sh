#!/bin/bash
# =============================================================================
# Build Script for enerby.dev
# Production build with optimizations
# =============================================================================

set -e

echo "🚀 Building enerby.dev for production..."

# Clean previous builds
echo "🧹 Cleaning previous builds..."
rm -rf dist/
rm -rf target/dx/

# Build release with SSG (P14-C1)
echo "🔨 Building with SSG..."
dx build --release

# Check if wasm-opt is available for additional optimization
if command -v wasm-opt &> /dev/null; then
    echo "⚡ Optimizing WASM with wasm-opt..."
    WASM_FILE=$(find dist -name "*.wasm" | head -1)
    if [ -n "$WASM_FILE" ]; then
        wasm-opt -Oz -o "$WASM_FILE.opt" "$WASM_FILE"
        mv "$WASM_FILE.opt" "$WASM_FILE"
        echo "   WASM optimized: $WASM_FILE"
    fi
else
    echo "⚠️  wasm-opt not found. Install with: cargo install wasm-opt"
    echo "   Skipping additional WASM optimization..."
fi

# Calculate build size
echo ""
echo "📊 Build Statistics:"
echo "===================="
if [ -d "dist" ]; then
    TOTAL_SIZE=$(du -sh dist | cut -f1)
    WASM_SIZE=$(find dist -name "*.wasm" -exec du -sh {} \; 2>/dev/null | cut -f1 || echo "N/A")
    HTML_SIZE=$(find dist -name "*.html" -exec du -sh {} \; 2>/dev/null | cut -f1 || echo "N/A")
    
    echo "Total dist size: $TOTAL_SIZE"
    echo "WASM size: $WASM_SIZE"
    echo "HTML size: $HTML_SIZE"
fi

echo ""
echo "✅ Build complete! Output in dist/"
echo ""
echo "To preview locally:"
echo "  python3 -m http.server 8000 -d dist"
echo ""
echo "To deploy:"
echo "  - Cloudflare Pages: Connect GitHub repo"
echo "  - Manual: Upload dist/ folder"
