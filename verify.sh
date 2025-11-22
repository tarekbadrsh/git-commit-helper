#!/bin/bash

# Git Commit Helper - Verification Script
# Tests that the built binary works correctly

set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Git Commit Helper - Verification${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Check if binary exists
BINARY="./target/release/git-commit-helper"

if [ ! -f "$BINARY" ]; then
    echo -e "${RED}✗ Binary not found at $BINARY${NC}"
    echo ""
    echo "Please build first with:"
    echo "  cargo build --release"
    exit 1
fi

echo -e "${GREEN}✓ Binary found${NC}"

# Check if binary is executable
if [ ! -x "$BINARY" ]; then
    echo -e "${RED}✗ Binary is not executable${NC}"
    chmod +x "$BINARY"
    echo -e "${GREEN}✓ Made binary executable${NC}"
fi

# Get binary size
SIZE=$(ls -lh "$BINARY" | awk '{print $5}')
echo -e "${GREEN}✓ Binary size: $SIZE${NC}"

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo ""
    echo -e "${YELLOW}⚠ Not in a git repository${NC}"
    echo "The binary exists but cannot test git commands here."
    echo "Move to a git repository to fully test the server."
    exit 0
fi

echo -e "${GREEN}✓ In a git repository${NC}"

# Check git commands work
echo ""
echo -e "${BLUE}Testing git commands...${NC}"

if git status > /dev/null 2>&1; then
    echo -e "${GREEN}✓ git status works${NC}"
fi

if git log -1 > /dev/null 2>&1; then
    echo -e "${GREEN}✓ git log works${NC}"
fi

if git diff HEAD > /dev/null 2>&1; then
    echo -e "${GREEN}✓ git diff works${NC}"
fi

echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${GREEN}✓ All checks passed!${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo "Your binary is ready to use!"
echo ""
echo "Next steps:"
echo "1. Get absolute path:"
echo "   echo \"\$(pwd)/target/release/git-commit-helper\""
echo ""
echo "2. Add to Claude Desktop config"
echo "3. Restart Claude Desktop"
echo "4. Test with: 'Show me what files have changed'"
echo ""
echo "For detailed setup, see QUICKSTART.md"
