fn sum(v: &[int]) -> int {
    if v.len() == 0 {
        0
    } else {
        *v.get(0).unwrap() + sum(v.slice_from(1))
    }
}

fn main() {
    let mut v = vec!();
    println!("{}", sum(v.as_slice()));
    v.push(1);
    println!("{}", sum(v.as_slice()));
    v.push(2);
    println!("{}", sum(v.as_slice()));
    v.push(3);
    println!("{}", sum(v.as_slice()));
    v.push(4);
    println!("{}", sum(v.as_slice()));
    v.push(-3);
    println!("{}", sum(v.as_slice()));
}
