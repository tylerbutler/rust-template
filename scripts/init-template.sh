#!/usr/bin/env bash
#
# Initialize this template for a new project.
# Replaces all placeholder values with your project-specific information.
#
# Usage:
#   ./scripts/init-template.sh <project-name> <author-name> <author-email> <github-username>
#
# Example:
#   ./scripts/init-template.sh my-cli "Jane Doe" "jane@example.com" "janedoe"

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

usage() {
    echo "Usage: $0 <project-name> <author-name> <author-email> <github-username>"
    echo ""
    echo "Arguments:"
    echo "  project-name    Name of your project (e.g., my-cli)"
    echo "  author-name     Your full name (e.g., \"Jane Doe\")"
    echo "  author-email    Your email (e.g., jane@example.com)"
    echo "  github-username Your GitHub username (e.g., janedoe)"
    echo ""
    echo "Example:"
    echo "  $0 my-cli \"Jane Doe\" \"jane@example.com\" \"janedoe\""
    exit 1
}

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check arguments
if [ $# -lt 4 ]; then
    log_error "Missing required arguments"
    usage
fi

PROJECT_NAME="$1"
AUTHOR_NAME="$2"
AUTHOR_EMAIL="$3"
GITHUB_USERNAME="$4"

# Validate project name (lowercase, hyphens allowed, no spaces)
if [[ ! "$PROJECT_NAME" =~ ^[a-z][a-z0-9-]*$ ]]; then
    log_error "Invalid project name: '$PROJECT_NAME'"
    echo "Project name must start with a lowercase letter and contain only lowercase letters, numbers, and hyphens."
    exit 1
fi

# Convert project name to different formats
PROJECT_NAME_UNDERSCORE="${PROJECT_NAME//-/_}"
CURRENT_YEAR=$(date +%Y)

# Detect sed replacement tool (prefer sd if available)
if command -v sd &> /dev/null; then
    REPLACE_CMD="sd"
    log_info "Using 'sd' for replacements"
else
    log_warn "'sd' not found, falling back to 'sed'"
    REPLACE_CMD="sed"
fi

# Function to replace in file
replace_in_file() {
    local pattern="$1"
    local replacement="$2"
    local file="$3"

    if [ ! -f "$file" ]; then
        log_warn "File not found, skipping: $file"
        return
    fi

    if [ "$REPLACE_CMD" = "sd" ]; then
        sd "$pattern" "$replacement" "$file"
    else
        # Use sed with extended regex
        if [[ "$OSTYPE" == "darwin"* ]]; then
            sed -i '' -E "s|$pattern|$replacement|g" "$file"
        else
            sed -i -E "s|$pattern|$replacement|g" "$file"
        fi
    fi
}

# Function to replace in all relevant files
replace_all() {
    local pattern="$1"
    local replacement="$2"

    # Find all relevant files, excluding .git, target, and lock files
    while IFS= read -r -d '' file; do
        replace_in_file "$pattern" "$replacement" "$file"
    done < <(find . -type f \( \
        -name "*.rs" -o \
        -name "*.toml" -o \
        -name "*.yml" -o \
        -name "*.yaml" -o \
        -name "*.md" -o \
        -name "*.json" -o \
        -name "justfile" -o \
        -name "LICENSE-*" \
    \) -not -path "./.git/*" -not -path "./target/*" -not -name "Cargo.lock" -print0)
}

log_info "Initializing template for project: $PROJECT_NAME"
log_info "Author: $AUTHOR_NAME <$AUTHOR_EMAIL>"
log_info "GitHub: github.com/$GITHUB_USERNAME/$PROJECT_NAME"
echo ""

# Confirm before proceeding
read -p "Proceed with initialization? [y/N] " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    log_info "Aborted."
    exit 0
fi

echo ""

# Replace project name (order matters - do hyphenated first)
log_info "Replacing project name..."
replace_all "rust-template" "$PROJECT_NAME"
replace_all "rust_template" "$PROJECT_NAME_UNDERSCORE"

# Replace author information
log_info "Replacing author information..."
replace_all 'Your Name <your\.email@example\.com>' "$AUTHOR_NAME <$AUTHOR_EMAIL>"
replace_all "Your Name" "$AUTHOR_NAME"
replace_all "your\.email@example\.com" "$AUTHOR_EMAIL"

# Replace GitHub username/repository
log_info "Replacing repository URLs..."
replace_all "yourusername/rust-template" "$GITHUB_USERNAME/$PROJECT_NAME"
replace_all "yourusername" "$GITHUB_USERNAME"

# Update copyright year in licenses
log_info "Updating license copyright..."
replace_in_file "Copyright \(c\) [0-9]+" "Copyright (c) $CURRENT_YEAR" "LICENSE-MIT"

# Reset changelog
log_info "Resetting changelog..."
cat > CHANGELOG.md << 'EOF'
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial project setup
EOF

# Regenerate derived configs
log_info "Regenerating derived configs..."
if command -v python3 &> /dev/null; then
    python3 scripts/generate-cliff-configs.py 2>/dev/null || log_warn "Failed to regenerate cliff.toml"
    python3 scripts/generate-commitlint-config.py 2>/dev/null || log_warn "Failed to regenerate .commitlintrc.json"
else
    log_warn "python3 not found, skipping config regeneration"
    echo "Run 'just generate-configs' manually after installing Python."
fi

echo ""
log_info "Template initialization complete!"
echo ""
echo "Next steps:"
echo "  1. Review the changes: git diff"
echo "  2. Run checks: just check"
echo "  3. Commit: git add -A && git commit -m 'chore: initialize from template'"
echo "  4. (Optional) Remove template files:"
echo "     - TEMPLATE_SETUP.md"
echo "     - scripts/init-template.sh"
echo ""
echo "See TEMPLATE_SETUP.md for additional configuration options."
