TARGET = optimalteam_rust
DEBUG_PATH = target/debug
RELEASE_PATH = target/release


ifeq ($(OS),Windows_NT)     # is Windows_NT on XP, 2000, 7, Vista, 10...
    run_script := type list.txt | .\target\release\optimalteam_rust.exe
	run_debug_script := type list.txt | .\target\debug\optimalteam_rust.exe
else
    run_script := target/release/optimalteam_rust < list.txt
    run_debug_script := target/debug/optimalteam_rust < list.txt

endif

all:
	cargo build --release

build_debug:
	cargo build

run: 
	$(run_script)

run_debug:
	$(run_debug_script)

clean:
	cargo clean