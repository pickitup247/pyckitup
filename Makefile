default:
	cargo-web deploy --release
	cargo build
	cargo build --release
