use std::num::Zero;

enum Tree<T> {
    Nil,
    Node(T, Box<Vec<Tree<T>>>)
}

impl<T> Tree<T> {
    fn leaf(val: T) -> Tree<T> {
        Node(val, box vec!())
    }

    fn bintree(parent: T, left: Tree<T>, right: Tree<T>) -> Tree<T> {
        Node(parent, box vec!(left, right))
    }
}


impl<T: Null + Clone + Add<T, T>> Tree<T> {
    fn add(&self) -> T {
        match *self {
            Nil => Null::null(),
            Node(ref v, ref ch) => {
                let mut sum = v.clone();
                for c in ch.iter() {
                    sum = sum + c.add();
                }
                sum
            }
        }
    }
}

trait Null {
    fn null() -> Self;
    fn is_null(&self) -> bool;
}

// I wanted to do impl<T: Zero> Null for T
// however I get a "conflicting implementation" error
// with the Vec<T> implementation, despite the fact that
// no Vec<T> implements Zero. I was looking at a github issue for this yesterday,
// so it's a known problem
impl Null for uint {
    fn null() -> uint { Zero::zero() }

    fn is_null(&self) -> bool {
        self.is_zero()
    }
}

impl<T: Eq> Null for Vec<T> {
    fn null() -> Vec<T> {
        vec!()
    }

    fn is_null(&self) -> bool {
        *self == vec!()
    }
}

fn main() {
    println!("{}", Nil::<uint>.add());
    let x = Node(1u, box vec!(Tree::leaf(2u), Tree::bintree(8u, Tree::leaf(3u), Tree::leaf(4u))));
    println!("{}", x.add());

    let v1 = vec!('a', 'b');
    let v2 = vec!('c');
    let v3 = vec!('d', 'e', 'f');
    let y: Tree<Vec<char>> = Tree::bintree(v1, Tree::leaf(v3), Tree::leaf(v2));
    println!("{}", y.add());
}
