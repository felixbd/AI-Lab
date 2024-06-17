##
# ai lab - GUI for annotating, training, and evaluating AI models, simplifying workflows
# Copyright (C) 2023  Felix Drees - GNU General Public License v3.0
#
# @file
# @version 0.1

#===========================================================================#
# You've stumbled upon a Makefile in a Rust project. Apologies to the Rust  #
# community — I'm just a Makefile in a world of cargo. But hey, if it ain't #
# broke, don't cargo it, right? ;) Now, let's make something!               #
#===========================================================================#

all: build run

build:
	cargo build --release

.PHONY: run
run:
	cargo run --release

clean:
	cargo clean
	# rm -fr ./docs/

test:
	cargo test

checkstyle:
	cargo clippy
	cargo fmt --all -- --check

doc:
	cargo doc --no-deps
	# rm -fr ./docs/
	# cp -r ./target/doc/ ./docs/  # for github pages

# Check for xdg-open and open doc
.PHONY: open_docs
open-doc:
	@if command -v xdg-open >/dev/null 2>&1; then \
		# xdg-open is installed. Opening docs ... \
		xdg-open ./target/doc/ai_lab/index.html; \
	elif command -v firefox >/dev/null 2>&1; then \
		# xdg-open is not installed. Using firefox to open docs ... \
		firefox ./target/doc/ai_lab/index.html; \
	else \
		echo "Neither xdg-open nor firefox is installed. Please install one to open docs via this Makefile."; \
	fi

format:
	cargo fmt

# end
