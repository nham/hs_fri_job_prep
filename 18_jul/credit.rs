use std::io::{BufferedReader, File, IoResult, Lines};
use std::collections::HashMap;

fn find_pair(items: Vec<uint>, credit: uint) -> Option<(uint, uint)> {
    let mut map = HashMap::new();

    for (i, &item) in items.iter().enumerate() {
        let complement: uint = credit - item;
        if map.contains_key(&complement) {
            let comp_ind = *map.find(&complement).unwrap();
            if i <= comp_ind {
                return Some((i, comp_ind));
            } else {
                return Some((comp_ind, i));
            }
        } else {
            map.insert(item, i);
        }
    }

    return None
}

#[test]
fn test_find_pair() {
    let v = vec!(0, 15, 3, 9, 7, 317, 28, 4);

    let mut res = find_pair(v.clone(), 18);
    assert!(res.is_some() && res.unwrap() == (1, 2));

    res = find_pair(v.clone(), 11);
    assert!(res.is_some() && res.unwrap() == (4, 7));

    res = find_pair(v.clone(), 14);
    assert!(res.is_none());

    res = find_pair(vec!(), 5);
    assert!(res.is_none());

    res = find_pair(vec!(2,5,5,7), 10);
    assert!(res.is_some() && res.unwrap() == (1, 2));
}

struct TestCase {
    credit: uint,
    items: String,
}

fn parse_input(path: &Path) -> Vec<TestCase> {
    let mut file = match File::open(path) {
        Err(why) => fail!("couldn't open {}: {}", path.display(), why.desc),
        Ok(file) => file,
    };

    let mut buffer = BufferedReader::new(file);
    let mut vec = vec!();

    buffer.read_line(); // discard the first line
    let mut lines: Lines<BufferedReader<File>> = buffer.lines();
    loop {
        match lines.next() {
            None => break,
            Some(result) => {
                let credit_string: String = result.unwrap();
                let credit: uint = from_str(credit_string.as_slice()).unwrap();
                lines.next();
                let items = lines.next().unwrap().unwrap();
                vec.push( TestCase { credit: credit, items: items } );
            }
        }
    }

    vec
}

fn main() {
}
