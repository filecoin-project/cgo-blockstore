.PHONY: build
build: example

HEADERS=libcgobs_example.h
LIBS=libcgobs_example.a

libcgobs_example.h: libcgobs_example.a
	cd rust && cbindgen -l C > ../$@

.PHONY: libcgobs_example.a
libcgobs_example.a:
	cd rust && cargo -Z unstable-options build --out-dir $(PWD)

example: $(HEADERS) $(LIBS)
	go build
