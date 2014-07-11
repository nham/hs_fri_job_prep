fn last_index_of<T: Eq + Copy>(list: &[T], val: T) -> Option<uint> {
    if list.len() == 0 {
        None
    } else {
        let x = *list.get(0).unwrap();
        let rest = last_index_of(list.slice_from(1), val);
        if x == val && rest.is_none() {
            Some(0u)
        } else if rest.is_some() {
            Some(rest.unwrap() + 1)
        } else {
            None
        }
    }
}

fn main() {
    let mut v = vec!();
    println!("{}", last_index_of(v.as_slice(), 'a'));
    v.push('a');
    println!("{}", last_index_of(v.as_slice(), 'a'));
    v.push('b');
    println!("{}", last_index_of(v.as_slice(), 'a'));
    v.push('c');
    println!("{}", last_index_of(v.as_slice(), 'a'));
    v.push('a');
    println!("{}", last_index_of(v.as_slice(), 'a'));
}
