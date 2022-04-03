run_dev:
	cargo watch -x run 

build_dev: 
	cargo build && cargo-watch -x run
