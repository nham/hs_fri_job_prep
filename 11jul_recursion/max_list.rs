//fn max(v: Vec<int>) -> Option<int> {
fn max(v: &[int]) -> Option<int> {
    if v.len() == 0 {
        None
    } else if v.len() == 1 {
        Some(*v.get(0).unwrap())
    } else {
        let x: int = *v.get(0).unwrap();
        let y: int = max( v.slice_from(1) ).unwrap();
        if x <= y {
            Some(y)
        } else {
            Some(x)
        }
    }
}

fn main () {
    let mut v = vec!();
    println!("{}", max(v.as_slice()));
    v.push(5);
    println!("{}", max(v.as_slice()));
    v.push(12);
    println!("{}", max(v.as_slice()));
    v.push(11);
    println!("{}", max(v.as_slice()));
    v.push(29);
    println!("{}", max(v.as_slice()));
    v.push(-44444);
    println!("{}", max(v.as_slice()));
    v.push(444);
    println!("{}", max(v.as_slice()));
    v.push(444);
    println!("{}", max(v.as_slice()));
}
