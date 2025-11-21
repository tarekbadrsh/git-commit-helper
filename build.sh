#!/bin/bash

# Git Commit Helper - Build Script
# Builds binaries for all platforms or just the current one

set -e  # Exit on error

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Git Commit Helper - Build Script${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Check if --all flag is provided
BUILD_ALL=false
if [[ "$1" == "--all" ]]; then
    BUILD_ALL=true
    echo -e "${YELLOW}Building for all platforms...${NC}"
    echo ""
fi

# Function to build for a specific target
build_target() {
    local target=$1
    local description=$2
    
    echo -e "${BLUE}Building for ${description}...${NC}"
    
    # Add target if not already added
    if ! rustup target list | grep -q "${target} (installed)"; then
        echo -e "${YELLOW}Adding target ${target}...${NC}"
        rustup target add "$target" || {
            echo -e "${RED}Failed to add target ${target}${NC}"
            return 1
        }
    fi
    
    # Build for the target
    if cargo build --release --target "$target"; then
        echo -e "${GREEN}✓ Successfully built for ${description}${NC}"
        echo -e "  Binary: target/${target}/release/git-commit-helper"
        echo ""
        return 0
    else
        echo -e "${RED}✗ Failed to build for ${description}${NC}"
        echo ""
        return 1
    fi
}

if [ "$BUILD_ALL" = true ]; then
    # Build for all platforms
    echo -e "${YELLOW}This will build binaries for:${NC}"
    echo "  • Linux x86_64"
    echo "  • macOS x86_64 (Intel)"
    echo "  • macOS ARM64 (Apple Silicon)"
    echo "  • Windows x86_64"
    echo ""
    
    FAILED=()
    
    # Linux x86_64
    build_target "x86_64-unknown-linux-gnu" "Linux x86_64" || FAILED+=("Linux x86_64")
    
    # macOS x86_64
    build_target "x86_64-apple-darwin" "macOS Intel" || FAILED+=("macOS Intel")
    
    # macOS ARM64
    build_target "aarch64-apple-darwin" "macOS Apple Silicon" || FAILED+=("macOS Apple Silicon")
    
    # Windows x86_64
    build_target "x86_64-pc-windows-gnu" "Windows x86_64" || FAILED+=("Windows x86_64")
    
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}  Build Summary${NC}"
    echo -e "${BLUE}========================================${NC}"
    
    if [ ${#FAILED[@]} -eq 0 ]; then
        echo -e "${GREEN}✓ All builds successful!${NC}"
        echo ""
        echo "Binaries are located in:"
        echo "  • target/x86_64-unknown-linux-gnu/release/git-commit-helper"
        echo "  • target/x86_64-apple-darwin/release/git-commit-helper"
        echo "  • target/aarch64-apple-darwin/release/git-commit-helper"
        echo "  • target/x86_64-pc-windows-gnu/release/git-commit-helper.exe"
    else
        echo -e "${RED}✗ Some builds failed:${NC}"
        for platform in "${FAILED[@]}"; do
            echo "  • $platform"
        done
        exit 1
    fi
else
    # Build for current platform only
    echo -e "${YELLOW}Building for current platform...${NC}"
    echo ""
    
    cargo build --release
    
    echo ""
    echo -e "${GREEN}✓ Build successful!${NC}"
    echo ""
    echo "Binary location:"
    echo "  target/release/git-commit-helper"
    echo ""
    echo "To build for all platforms, run:"
    echo "  ./build.sh --all"
fi

echo ""
echo -e "${GREEN}Done!${NC}"
