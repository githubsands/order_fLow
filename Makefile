.PHONY:
build_darwin_x86:
	rm -rf .cargo/config.toml
	echo '[target.x86_64-apple-darwin]' >> .cargo/config.toml
	echo 'rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]' >> .cargo/config.toml
	carog

.PHONY:
build_linux_x86:
	rm -rf .cargo/config.toml
	echo '[target.x86_64-unknown-linux-gnu]' >> .cargo/config.toml
	echo 'rustflags = ["-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]' >> .cargo/config.toml

.PHONY:
cargo_install_tools:
	rustup toolchain install nightly --allow-downgrade
	rustup component add rustfmt
	rustup component add clippy
	cargo install cargo-expand

.PHONY:
cargo_install_tools_darwin:
	brew install michaeleisel/zld/zld

.PHONY:
cargo_install_tools_ubuntu:
	sudo apt-get install lld clang

# cargo_check_macros uses the nightly compiler to check macros
.PHONY:
cargo_check_macros:
	cargo +nightly expand

.PHONY: check-vars
check-vars:
ifndef PACKAGE
	$(error PACKAGE variable was not defined)
endif
ifndef VERSION
	$(error VERSION variable was not defined)
endif

.PHONY:
cargo_add_package: check-vars
	cargo add $(PACKAGE):$(VERSION)

.PHONY:
cargo_install:
ifndef TOOL
	$(error TOOL variable was not defined)
endif
	cargo install $(TOOL)

.PHONY:
run_test:
	cargo test

.PHONY:
watch:
	cargo watch -x check

.PHONY:
format:
	cargo fmt

.PHONY:
lint:
	cargo clippy -- -D warnings

.PHONY:
scan_dependency_tree:
	cargo audit
