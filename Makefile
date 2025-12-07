INSTALL := install
INSTALL_DIR := $(HOME)/.local/bin

BUILD_DIR := target/release
OUT := ciryl
SCRIPT := name_lyric.sh

.PHONY: install install-script clean

$(BUILD_DIR)/$(OUT):
	cargo build --release

install: $(BUILD_DIR)/$(OUT)
	install -m 755 $(BUILD_DIR)/$(OUT) $(INSTALL_DIR)/$(OUT)

install-script:
	install -m 755 $(SCRIPT) $(INSTALL_DIR)/$(SCRIPT)

clean:
	cargo clean
