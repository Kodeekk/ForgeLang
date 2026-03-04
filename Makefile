PREFIX ?= /usr/local
BINDIR = $(PREFIX)/bin
STDLIBDIR = $(HOME)/.forgelang/stdlib

.PHONY: all build install uninstall clean

all: build

build:
	cargo build --release

# - Binaries go to $(PREFIX)/bin
# - Stdlib goes to ~/.forgelang/stdlib (symlinked from source)
install: build
	@echo "Installing ForgeLang..."
	@mkdir -p $(BINDIR)
	@mkdir -p $(HOME)/.forgelang
	@# Remove old symlink if exists
	@rm -f $(STDLIBDIR)
	@# Create symlink to stdlib
	@ln -s $(CURDIR)/stdlib $(STDLIBDIR)
	@echo "Created symlink: $(STDLIBDIR) -> $(CURDIR)/stdlib"
	@# Install binaries
	sudo cp target/release/fl $(BINDIR)/fl
	sudo cp target/release/maul $(BINDIR)/maul
	@echo "Installed binaries to $(BINDIR)"
	@echo ""
	@echo "ForgeLang installed successfully!"
	@echo "  fl     -> $(BINDIR)/fl"
	@echo "  maul   -> $(BINDIR)/maul"
	@echo "  stdlib -> $(STDLIBDIR) (symlink)"
	@echo ""
	@echo "Make sure $(BINDIR) is in your PATH"

uninstall:
	@echo "Uninstalling ForgeLang..."
	sudo rm -f $(BINDIR)/fl
	sudo rm -f $(BINDIR)/maul
	@rm -rf $(HOME)/.forgelang
	@echo "ForgeLang uninstalled"

clean:
	cargo clean
	@rm -rf target
