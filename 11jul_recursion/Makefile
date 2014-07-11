fib: build_dir fib.rs
	rustc fib.rs --out-dir build
	./build/fib

fib-bench: build_dir fib.rs
	rustc --test fib.rs --out-dir build
	./build/fib --bench

max_list: max_list.rs build_dir
	rustc max_list.rs --out-dir build
	./build/max_list

.PHONY: build_dir

build_dir:
	mkdir -p build
