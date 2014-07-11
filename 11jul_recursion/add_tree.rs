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


impl<T: Zero + Copy + Add<T, T>> Tree<T> {
    fn add(&self) -> T {
        match *self {
            Nil => Zero::zero(),
            Node(v, ref ch) => {
                let mut sum = v;
                for c in ch.iter() {
                    sum = sum + c.add();
                }
                sum
            }
        }
    }
}

fn main() {
    println!("{}", Nil::<uint>.add());
    let x = Node(1u, box vec!(Tree::leaf(2u), Tree::bintree(8u, Tree::leaf(3u), Tree::leaf(4u))));
    println!("{}", x.add());
}
