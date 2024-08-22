set windows-shell := ["nu.exe", "-c"]

bin := "warband"

watch:
    cargo watch --features 'dev' -q -c -x 'run -- --bin {{bin}}'

dev:
    cargo run --bin {{bin}} --features 'dev'

run:
    cargo run --bin {{bin}} --release

run-wasm:
    cargo run --bin {{bin}} --target wasm32-unknown-unknown --features 'dev'

build:
    cargo build --bin {{bin}} --release

build-wasm:
    cargo build --bin {{bin}} --profile wasm --target wasm32-unknown-unknown

build-debug:
    cargo build --bin {{bin}} --features 'dev'

fix:
	just clippy
	just fmt

fmt: 
    cargo fmt -v --all

clippy:
    cargo clippy --fix --workspace --all-features --allow-dirty --allow-no-vcs -- --no-deps

ci: 
    cargo run -p ci

clean: 
    cargo clean
