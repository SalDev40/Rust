#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
use std::fmt::{Debug, Display};
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    left: BinTree<T>,
    right: BinTree<T>,
}

#[derive(Debug)]
pub enum BinTree<T> {
    NonEmptyTree(Box<Node<T>>),
    EmptyTree,
}

impl<T> BinTree<T>
where
    T: PartialOrd + Debug + Display + Copy,
{
    fn new() -> Self {
        return BinTree::EmptyTree;
    }
    fn add(&mut self, data: T) {
        match &mut *self {
            BinTree::NonEmptyTree(node) => {
                if data <= node.data {
                    println!("GOING LEFT {:?} (data) <= {:?} (node)", data, node.data);
                    node.left.add(data)
                } else {
                    println!("GOING RIGHT {:?} (node) < {:?} (data)", node.data, data);
                    node.right.add(data)
                }
            }
            BinTree::EmptyTree => {
                *self = BinTree::NonEmptyTree(Box::new(Node {
                    data: data,
                    left: BinTree::EmptyTree,
                    right: BinTree::EmptyTree,
                }));
            }
        }
    }

    fn findItem(&self, item: T) -> Result<bool, Box<dyn std::error::Error>> {
        let found: bool = false;
        match &*self {
            BinTree::NonEmptyTree(node) => {
                if node.data != item {
                    if item <= node.data {
                        println!("SEARCHING LEFT {:?} (item) <= {:?} (node)", item, node.data);
                        node.left.findItem(item)?;
                    } else {
                        println!("SEARCHING RIGHT {:?} (node) < {:?} (item)", node.data, item);
                        //propogate error up recursively
                        node.right.findItem(item)?;
                    }
                } 
                return Ok(true);
            }
            BinTree::EmptyTree => {
                println!("TREE EMPTY!");
                return Err(Box::new(Error::new(
                    ErrorKind::Other,
                    "find() -> ITEM NOT FOUND",
                )));
            }
        }
    }

    fn removeItem(&mut self, data: T) {
        
    }
}

pub fn printLog(x: i32, tree: &BinTree<i32>) {
    println!("ADDED: {:?}", x);
    // println!("CURRENT TREE -> {:#?} \n", tree);
}

pub fn binMain() {
    let mut tree: BinTree<i32> = BinTree::new();
    tree.add(10);
    printLog(10, &tree);

    tree.add(20);
    printLog(20, &tree);

    tree.add(5);
    printLog(5, &tree);

    tree.add(16);
    printLog(16, &tree);

    println!("{:#?}", tree);
    println!("found -> {:#?}", tree.findItem(3).expect("FIND"));

    // tree.add(2);

    // println!("CURRENT TREE -> {:#?}", tree);
}
