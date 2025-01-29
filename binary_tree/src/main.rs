// implementacao simples de arvore binaria para entender conceitos de smart pointers, tipo Box e deref.

use std::cmp::Ordering;

struct BinaryTree {
    value : i32,
    right : Option<Box<BinaryTree>>,
    left : Option<Box<BinaryTree>>,
}

impl BinaryTree {
    fn new(value: i32) -> Self {
        Self {
            value: value,
            right: None,
            left: None
        }
    }

    fn put(&mut self, new_value: i32) {
        match self.value.cmp(&new_value) {
            Ordering::Less => match self.right.as_mut() {
                None => self.right = Some(Box::new(BinaryTree::new(new_value))),
                Some(n) => n.put(new_value),
            },
            Ordering::Equal => return,
            Ordering::Greater => match self.left.as_mut() {
                None => self.left = Some(Box::new(BinaryTree::new(new_value))),
                Some(n) => n.put(new_value),
            }
        }
    }

    fn get(&self, query: i32) -> bool {
        match self.value.cmp(&query) {
            Ordering::Less => match self.right.as_deref() {
                None => false,
                Some(n) => n.get(query),
            }            ,
            Ordering::Equal => true,
            Ordering::Greater => match self.left.as_deref() {
                None => false,
                Some(n) => n.get(query)                
            }
        }
    }

    fn print(&self) {
        print!("(");
        match self.left.as_deref() {
            None => { },
            Some(n) => n.print(),
        }

        print!(" {} ", self.value);

        match self.right.as_deref() {
            None => { },
            Some(n) => n.print(),
        }
        print!(")");
    }
    
}

fn main() {
    let mut tree = BinaryTree::new(20);

    tree.put(25);
    tree.put(14);
    tree.put(22);
    tree.put(23);
    tree.put(3);

    tree.print();
    println!();

    let ok = tree.get(3);
    let nok = tree.get(12);

    println!("3: {ok}, 12: {nok}");
}
