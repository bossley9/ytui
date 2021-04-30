BINARY=ytui

PREFIX=/usr/local
BIN=$(PREFIX)/bin
TARGET=target
RELEASE=$(TARGET)/release

all:
	cargo build --release

install:
	@mkdir -p $(BIN)
	@mv $(RELEASE)/$(BINARY) $(BIN)/$(BINARY)

check:
	cargo check

debug:
	cargo build

clean:
	@rm -rf $(TARGET)

test:
	cargo test

uninstall:
	@rm -f $(BIN)/$(BINARY)

.PHONY: all install check debug clean test uninstall
