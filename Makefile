BIN := gw

.PHONY: all build release test lint fmt fmt-check deny dist dist-plan docker run clean control-plane control-plane-test control-plane-integration

all: fmt lint test build

build:
	cargo build --workspace

release:
	cargo build --release -p gw-server --locked

test:
	cargo test --workspace

lint:
	cargo clippy --workspace --all-targets -- -D warnings

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

deny:
	cargo deny check

# dist owns releases (v* tags in CI); these mirror it locally.
dist-plan:
	dist plan

dist:
	dist build

docker:
	docker build -t gateway .

run:
	cargo run -p gw-server

control-plane:
	$(MAKE) -C control-plane build

control-plane-test:
	$(MAKE) -C control-plane test web-test

control-plane-integration:
	$(MAKE) -C control-plane test-integration

clean:
	cargo clean
