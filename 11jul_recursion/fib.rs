extern crate test;

use std::collections::hashmap::HashMap;
use test::Bencher;

fn naive_fib(n: uint) -> uint {
    if n < 2 {
        n
    } else {
        naive_fib(n-1) + naive_fib(n-2) 
    }
}


struct FibMemoizer {
    mem: HashMap<uint, uint>,
}

impl FibMemoizer {
    fn new() -> FibMemoizer {
        FibMemoizer { mem: HashMap::new() }
    }

    fn memoizing_fib(&mut self, n: uint) -> uint {
        if !self.mem.contains_key(&n) {
            let res = self.fib(n);
            self.mem.insert(n, res);
            res
        } else {
            *self.mem.find(&n).unwrap()
        }
    }

    fn fib(&mut self, n: uint) -> uint {
        if n < 2 {
            n
        } else {
            self.memoizing_fib(n-1) + self.memoizing_fib(n-2) 
        }
    }
}

#[bench]
fn bench_naive(b: &mut Bencher) {
    b.iter(|| naive_fib(25) );
}

#[bench]
fn bench_memoizing(b: &mut Bencher) {
    let mut fm = FibMemoizer::new();
    b.iter(|| fm.fib(25) );
}

fn main() {
    let mut fm = FibMemoizer::new();

    for i in range(0u, 20) {
        println!("Naive: {}, Memoizing: {}", naive_fib(i), fm.fib(i));
    }
}
