RELEASE_DIR := libs
TEMP_DIR := temp

CARGO_BUILD := cargo build --release

MACOS_X86_DIR := $(RELEASE_DIR)/macos_x86
MACOS_ARM64_DIR := $(RELEASE_DIR)/macos_arm64
WINDOWS_DIR := $(RELEASE_DIR)/windows
LINUX_X86_DIR := $(RELEASE_DIR)/linux_x86

.PHONY: libs macos_x86 macos_arm64 windows lint lint_apply test build_test

libs: macos_x86 macos_arm64 windows

macos_x86:
	@echo "Building release version for macOS x86_64..."
	@mkdir -p $(TEMP_DIR) $(MACOS_X86_DIR)
	@CARGO_TARGET_DIR=$(TEMP_DIR) $(CARGO_BUILD) --target x86_64-apple-darwin
	@cp $(TEMP_DIR)/x86_64-apple-darwin/release/lib_turso_java.dylib $(MACOS_X86_DIR)
	@rm -rf $(TEMP_DIR)

macos_arm64:
	@echo "Building release version for macOS ARM64..."
	@mkdir -p $(TEMP_DIR) $(MACOS_ARM64_DIR)
	@CARGO_TARGET_DIR=$(TEMP_DIR) $(CARGO_BUILD) --target aarch64-apple-darwin
	@cp $(TEMP_DIR)/aarch64-apple-darwin/release/lib_turso_java.dylib $(MACOS_ARM64_DIR)
	@rm -rf $(TEMP_DIR)

# windows generates file with name `_turso_java.dll` unlike others, so we manually add prefix
windows:
	@echo "Building release version for Windows..."
	@mkdir -p $(TEMP_DIR) $(WINDOWS_DIR)
	@CARGO_TARGET_DIR=$(TEMP_DIR) $(CARGO_BUILD) --target x86_64-pc-windows-gnu
	@cp $(TEMP_DIR)/x86_64-pc-windows-gnu/release/_turso_java.dll $(WINDOWS_DIR)/lib_turso_java.dll
	@rm -rf $(TEMP_DIR)

linux_x86:
	@echo "Building release version for linux x86_64..."
	@mkdir -p $(TEMP_DIR) $(LINUX_X86_DIR)
	@CARGO_TARGET_DIR=$(TEMP_DIR) $(CARGO_BUILD) --target x86_64-unknown-linux-gnu
	@cp $(TEMP_DIR)/x86_64-unknown-linux-gnu/release/lib_turso_java.so $(LINUX_X86_DIR)
	@rm -rf $(TEMP_DIR)

lint:
	./gradlew spotlessCheck

lint_apply:
	./gradlew spotlessApply

test: lint build_test
	./gradlew test

build_test:
	CARGO_TARGET_DIR=src/test/resources/turso cargo build

publish_local:
	./gradlew clean publishToMavenLocal
