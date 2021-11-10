.PHONY: bench
bench: $(HEADERS) $(LIBS)
	go build

HEADERS=libcgobs_example.h
LIBS=libcgobs_example.a
CARGO_TARGET_DIR ?= "target"

libcgobs_example.h: libcgobs_example.a
	cd rust && cbindgen -l C > ../$@

.PHONY: libcgobs_example.a
libcgobs_example.a:
	cd rust && cargo build
	cp "$(CARGO_TARGET_DIR)/debug/libcgobs_example.a" .

.PHONY: bench
bench: $(HEADERS) $(LIBS)
	go test -bench .
