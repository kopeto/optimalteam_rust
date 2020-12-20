all:
	cargo build --release

build_debug:
	cargo build

run: 
	target/release/optimalteam_rust < list.txt

run_debug:
	target/debug/optimalteam_rust < list.txt

clean:
	cargo clean