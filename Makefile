VERSION=$(shell grep ^version Cargo.toml | cut -d \" -f 2)
DIST_FILES=src/main.rs src/syno/*.rs src/ui/*.rs Cargo.toml ChangeLog \
	COPYING README.md Makefile

.PHONY: all
all:
	cargo build

dist: synodl-$(VERSION).tar.gz

synodl-%.tar.gz: $(DIST_FILES)
	tar czf $@ $^

.PHONY: clean
clean:
	rm -f synodl-*.tar.gz
	cargo clean
