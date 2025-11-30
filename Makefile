# Forge Protocol - Build & Documentation
#
# Targets for building the CLI and generating presentation PDFs

.PHONY: help build build-static build-compressed install-system install-user \
        presentation presentation-pdf presentation-pptx lint lint-fix \
        install-tools clean distclean

# Check if marp-cli is installed
HAS_MARP := $(shell command -v marp 2> /dev/null)

# Binary name and paths
BINARY := asimov
RELEASE_BIN := cli/target/release/$(BINARY)
MUSL_BIN := cli/target/x86_64-unknown-linux-musl/release/$(BINARY)

help:
	@echo "🔥 Forge Protocol - Available Commands"
	@echo ""
	@echo "Build:"
	@echo "  make build              - Build release binary"
	@echo "  make build-static       - Build static musl binary"
	@echo "  make build-compressed   - Build UPX-compressed static binary"
	@echo ""
	@echo "Install:"
	@echo "  make install-system     - Install to /usr/local/bin (requires sudo)"
	@echo "  make install-user       - Install to ~/.local/bin"
	@echo ""
	@echo "Presentation:"
	@echo "  make presentation       - Generate PDF presentation"
	@echo "  make presentation-pdf   - Generate PDF presentation"
	@echo "  make presentation-pptx  - Generate PowerPoint presentation"
	@echo ""
	@echo "Quality:"
	@echo "  make lint               - Run markdownlint on all docs"
	@echo "  make lint-fix           - Auto-fix markdown issues"
	@echo ""
	@echo "Utilities:"
	@echo "  make install-tools      - Show installation commands"
	@echo "  make clean              - Remove generated files"
	@echo "  make distclean          - Remove all build artifacts"

# ==============================================================================
# Build Targets
# ==============================================================================

build:
	@echo "🔨 Building release binary..."
	cd cli && cargo build --release
	@echo "✅ Built: $(RELEASE_BIN)"
	@ls -lh $(RELEASE_BIN)

build-static:
	@echo "🔨 Building static musl binary..."
	cd cli && cargo build --release --target x86_64-unknown-linux-musl
	@echo "✅ Built: $(MUSL_BIN)"
	@ls -lh $(MUSL_BIN)

build-compressed: build-static
	@echo "📦 Compressing with UPX..."
	upx --best --lzma $(MUSL_BIN)
	@echo "✅ Compressed: $(MUSL_BIN)"
	@ls -lh $(MUSL_BIN)

# ==============================================================================
# Install Targets
# ==============================================================================

install-system: build-compressed
	@echo "📥 Installing to /usr/local/bin..."
	sudo install -m 755 $(MUSL_BIN) /usr/local/bin/$(BINARY)
	@echo "✅ Installed: /usr/local/bin/$(BINARY)"
	@asimov --version

install-user: build-compressed
	@echo "📥 Installing to ~/.local/bin..."
	@mkdir -p ~/.local/bin
	install -m 755 $(MUSL_BIN) ~/.local/bin/$(BINARY)
	@echo "✅ Installed: ~/.local/bin/$(BINARY)"
	@~/.local/bin/asimov --version

# ==============================================================================
# Presentation Targets
# ==============================================================================

presentation: presentation-pdf
	@echo ""
	@echo "✅ Presentation generated: Forge_Protocol_Suite.pdf"

presentation-pdf:
	@echo "📊 Generating PDF presentation..."
ifndef HAS_MARP
	@echo "⚠️  Marp CLI not found. Installing..."
	@npm install -g @marp-team/marp-cli
endif
	@marp docs/PRESENTATION.md -o Forge_Protocol_Suite.pdf --pdf --allow-local-files
	@echo "✅ Generated: Forge_Protocol_Suite.pdf"
	@ls -lh Forge_Protocol_Suite.pdf

presentation-pptx:
	@echo "📊 Generating PowerPoint presentation..."
ifndef HAS_MARP
	@echo "⚠️  Marp CLI not found. Installing..."
	@npm install -g @marp-team/marp-cli
endif
	@marp docs/PRESENTATION.md -o Forge_Protocol_Suite.pptx --pptx --allow-local-files
	@echo "✅ Generated: Forge_Protocol_Suite.pptx"
	@ls -lh Forge_Protocol_Suite.pptx

lint:
	@echo "📝 Linting markdown files..."
	@npx markdownlint-cli2 '**/*.md'
	@echo "✅ Markdown lint passed"

lint-fix:
	@echo "🔧 Fixing markdown issues..."
	@npx markdownlint-cli2 '**/*.md' --fix
	@echo "✅ Markdown fixes applied"

install-tools:
	@echo "📦 Required tools:"
	@echo ""
	@echo "1. Marp CLI (presentation generation)"
	@echo "   npm install -g @marp-team/marp-cli"
	@echo ""
	@echo "2. markdownlint-cli2 (documentation validation)"
	@echo "   npm install -g markdownlint-cli2"
	@echo ""
	@echo "Current status:"
	@command -v marp >/dev/null 2>&1 && echo "  ✅ Marp CLI installed" || echo "  ❌ Marp CLI not found"
	@command -v npx >/dev/null 2>&1 && echo "  ✅ npx available" || echo "  ❌ npx not found"

clean:
	@echo "🧹 Cleaning generated files..."
	@rm -f Forge_Protocol_Suite.pdf Forge_Protocol_Suite.pptx
	@echo "✅ Clean complete"

distclean: clean
	@echo "🧹 Cleaning build artifacts..."
	cd cli && cargo clean
	@echo "✅ Distclean complete"
