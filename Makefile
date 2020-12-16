BINARY=ytui

PREFIX=/usr/local
BIN=$(PREFIX)/bin
TARGET=target
RELEASE=$(TARGET)/release

CP=cp -f
RM=rm -rf
MKDIR_P=mkdir -p

all:
	cargo build --release

install: all
	$(MKDIR_P) $(BIN)
	$(CP) $(RELEASE)/$(BINARY) $(BIN)/$(BINARY)

clean:
	$(RM) $(TARGET)

uninstall:
	$(RM) $(BIN)/$(BINARY)

.PHONY: all install clean uninstall

