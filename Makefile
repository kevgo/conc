RTA_VERSION = 0.26.1  # run-that-app version to use

fix: tools/rta@${RTA_VERSION}  # auto-corrects issues
	cargo +nightly fix --allow-dirty
	cargo clippy --fix --allow-dirty
	cargo +nightly fmt
	tools/rta dprint fmt

help:  # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.PHONY' | grep -v '.SILENT:' | grep '#' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

lint: tools/rta@${RTA_VERSION}  # runs all linters
	cargo clippy --all-targets --all-features -- --deny=warnings
	git diff --check

setup:  # install development dependencies on this computer
	rustup component add clippy
	rustup toolchain add nightly
	rustup component add rustfmt --toolchain nightly

test: fix unit lint  # runs all tests

todo:  # displays all TODO items
	@git grep --color=always --line-number TODO ':!target' | grep -v Makefile

unit:  # runs the unit tests
	cargo test --locked

update:  # updates the dependencies
	cargo install cargo-edit cargo-machete
	cargo machete
	cargo upgrade
	cargo run -- --update
	tools/rta --update

# --- HELPER TARGETS --------------------------------------------------------------------------------------------------------------------------------

tools/rta@${RTA_VERSION}:
	@rm -f tools/rta*
	@(cd tools && curl https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh -s ${RTA_VERSION})
	@mv tools/rta tools/rta@${RTA_VERSION}
	@ln -s rta@${RTA_VERSION} tools/rta

.DEFAULT_GOAL := help
.SILENT:
