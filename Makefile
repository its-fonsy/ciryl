INSTALL := install
INSTALL_DIR := $(HOME)/.local/bin

BUILD_DIR := target/release
OUT := ciryl

.PHONY: install release clean

$(BUILD_DIR)/$(OUT):
	cargo build --release

install: $(BUILD_DIR)/$(OUT)
	install -m 755 $(BUILD_DIR)/$(OUT) $(INSTALL_DIR)/$(OUT)

clean:
	cargo clean
