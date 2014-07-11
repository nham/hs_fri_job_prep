fib: build_dir
	rustc fib.rs --out-dir build
	./build/fib

fib-bench: build_dir
	rustc --test fib.rs --out-dir build
	./build/fib --bench

.PHONY: build_dir

build_dir:
	mkdir -p build
