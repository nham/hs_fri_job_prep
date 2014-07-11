fib: build_dir fib.rs
	rustc fib.rs --out-dir build
	./build/fib

fib-bench: build_dir fib.rs
	rustc --test fib.rs --out-dir build
	./build/fib --bench

max_list: max_list.rs build_dir
	rustc max_list.rs --out-dir build
	./build/max_list

sum: sum.rs build_dir
	rustc sum.rs --out-dir build
	./build/sum

last_index_of: last_index_of.rs build_dir
	rustc last_index_of.rs --out-dir build
	./build/last_index_of

.PHONY: build_dir

build_dir:
	mkdir -p build
