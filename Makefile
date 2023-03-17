VERSION=$(shell grep ^version Cargo.toml | cut -d \" -f 2)
DIST_FILES=src/main.rs src/syno/*.rs src/ui/*.rs Cargo.toml ChangeLog \
	COPYING README.md Makefile

.PHONY: all
all:
	cargo build

dist: synodl-$(VERSION).tar.gz

synodl-%.tar.gz: $(DIST_FILES)
	tar czf $@ --transform 's:^:synodl-$(VERSION)/:' $^

.PHONY: clean
clean:
	rm -f synodl-*.tar.gz
	cargo clean

.PHONY: check
check:
	cargo test

.PHONY: coverage
coverage:
	CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' \
			  LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' \
			  cargo test
	grcov . --binary-path ./target/debug/deps/ -s . -t html --branch \
		-o target/coverage \
		--llvm-path=/usr/bin
