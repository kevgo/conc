RTA_VERSION = 0.36.0  # run-that-app version to use

RTA          = tools/rta@${RTA_VERSION}
DPRINT       = $(RTA) dprint
GHERKIN_LINT = $(NPM) exec --yes gherkin-lint
GHOKIN       = $(RTA) ghokin
NPM          = $(RTA) npm
RUMDL        = $(RTA) rumdl
TAPLO        = $(RTA) taplo

build:  # builds the codebase
	cargo build

cuke: build  # runs all end-to-end tests
	cargo test --test=cucumber

cukethis: build  # runs only end-to-end tests with a @this tag
	cargo test --test cucumber -- -t @this

fix: ${RTA}  # auto-corrects issues
	cargo +nightly fix --allow-dirty
	cargo clippy --fix --allow-dirty
	cargo +nightly fmt
	${DPRINT} fmt
	${GHOKIN} fmt replace features/
	$(RUMDL) fmt
	$(TAPLO) format

help:  # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.PHONY' | grep -v '.SILENT:' | grep '#' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

install:  # installs this tool on the local machine
	cargo install --locked --path .

lint: ${RTA}  # runs all linters
	cargo clippy -- -Wclippy::pedantic --deny=clippy::unwrap_used --deny=clippy::expect_used --deny=clippy::panic  # lint production code
	cargo clippy --all-targets --all-features -- --deny=warnings --allow=clippy::unwrap_used # lint all code including test code
	cargo clippy --test=cucumber --all-features -- --deny=warnings \
		--allow=clippy::unwrap_used
	git diff --check
	${GHERKIN_LINT}
	$(RUMDL) check
	# $(TAPLO) check  # current version has a bug with Cargo.toml, see https://github.com/rust-lang/cargo/issues/15030

ps: fix test  # runs all automations

setup: ${RTA}  # install development dependencies on this computer
	rustup component add clippy
	rustup toolchain add nightly
	rustup component add rustfmt --toolchain nightly
	${NPM} ci

test: fix unit lint cuke  # runs all tests

todo:  # displays all TODO items
	@git grep --color=always --line-number TODO ':!target' | grep -v Makefile

unit:  # runs the unit tests
	cargo test --locked
	cargo test -p test_helpers

update:  # updates the dependencies
	cargo install cargo-edit cargo-machete
	cargo machete
	cargo upgrade
	${RTA} --update

# --- HELPER TARGETS --------------------------------------------------------------------------------------------------------------------------------

${RTA}:
	@rm -f tools/rta*
	@(cd tools && curl https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh -s -- --version ${RTA_VERSION} --name rta@${RTA_VERSION})

.DEFAULT_GOAL := help
.SILENT:
