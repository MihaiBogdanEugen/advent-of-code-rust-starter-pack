## help: Prints this help message
help:
	@echo "\n==============\nAdvent of Code\n==============\nUsage: \n"
	@sed -n "s/^##//p" ${MAKEFILE_LIST} | column -t -s ":" |  sed -e "s/^/ /"

## build-debug: Build the local package and all of its dependencies
build-debug:
	cargo build

## run-debug: Build and run the current package
run-debug: build-debug
	RUST_BACKTRACE=full cargo run -- --year $(year) --day $(day)

## build: Build the local package and all of its dependencies with optimizations (release mode)
build:
	cargo build --release

## run: Build and run the current optimized package
run: build
	cargo run --release -- --year $(year) --day $(day)

## update: Update dependencies listed in Cargo.lock
update:
	cargo update

## check: Analyze the current package and report errors, but don't build object files
check:
	cargo check --verbose

## clean: Clean the current package and all build artifacts
clean:
	@rm -rdf Cargo.lock && cargo clean

## fmt: Format all Rust files of the current crate
fmt:
	cargo fmt --all

## test: Run the tests
test:
	cargo test --verbose

## clippy: Run cargo clippy for static ckecks
clippy:
	cargo clippy --all-targets --all-features --verbose

.PHONY: help build-debug run-debug build run update check clean fmt test clippy