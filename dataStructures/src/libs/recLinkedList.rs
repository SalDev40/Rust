#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::io::{Error, ErrorKind};

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    next: LinkedList<T>,
}

#[derive(Debug, Clone)]
enum LinkedList<T> {
    NonEmpty(Box<Node<T>>),
    Empty,
}

impl<T: std::fmt::Debug + std::fmt::Display + PartialOrd + Clone> LinkedList<T> {
    fn new() -> Self {
        return LinkedList::Empty;
    }

    fn add(&mut self, data: T) {
        match &mut *self {
            LinkedList::NonEmpty(node) => {
                node.next.add(data);
            }
            LinkedList::Empty => {
                *self = LinkedList::NonEmpty(Box::new(Node {
                    value: data,
                    next: LinkedList::Empty,
                }))
            }
        }
    }

    fn findItem(&self, item: T) -> Result<u64, Box<dyn std::error::Error>> {
        static mut i: u64 = 0;
        match &*self {
            LinkedList::NonEmpty(node) => {
                if node.value != item {
                    unsafe { i = i + 1 };
                    //recursively pass the error up to calling function
                    match node.next.findItem(item) {
                        Ok(loc) => {}
                        Err(e) => {
                            //recursively return the error
                            println!("error: {}", e);
                            //make sure to return error or else
                            //it will RETURN last line which is Ok(i)
                            //after leaving this if block
                            return Err(e);
                        }
                    };

                //shortcut
                // node.next.findItem(item)?;
                } else {
                    println!("{} found at position: {}", item, unsafe { i });
                }

                //recursively pass the location up to calling function
                return unsafe { Ok(i) };
            }
            LinkedList::Empty => {
                return Err(Box::new(Error::new(
                    ErrorKind::Other,
                    "finditem() -> ITEM NOT FOUND",
                )));
            }
        }
    }

    fn removeItem(&mut self, item: T) -> Result<bool, Box<dyn std::error::Error>> {
        match &mut *self {
            LinkedList::NonEmpty(node) => {
                println!("checking {:?}", node.value);
                //if moving head node
                //make start of list next node

                if node.value == item {
                    println!("removing {:?}", node.value);
                    *self = match &*self {
                        LinkedList::NonEmpty(node) => node.next.clone(),
                        LinkedList::Empty => LinkedList::Empty,
                    };
                    return Ok(true);
                }

                //check if next node is the one to remove
                let found: bool = match &node.next {
                    LinkedList::NonEmpty(nextNode) => {
                        if nextNode.value != item {
                            false
                        } else {
                            println!("node {:?} found after {:?}", item, node.value);
                            true
                        }
                    }
                    LinkedList::Empty => false,
                };

                //if not removing next node, move to next node
                if !found {
                    //if error -> propogate error up recursively
                    node.next.removeItem(item)?;
                } else {
                    //if removing next node
                    //make node.next = node.next.next
                    node.next = match &node.next {
                        LinkedList::NonEmpty(node) => node.next.clone(),
                        LinkedList::Empty => LinkedList::Empty,
                    };
                    println!("removing node {:?} after {:?}", item, node.value);
                }
                return Ok(true);
            }
            LinkedList::Empty => {
                return Err(Box::new(Error::new(
                    ErrorKind::Other,
                    "removeItem() -> ITEM NOT REMOVED",
                )));
            }
        }
    }
}

pub fn recLinkMain() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.add(5);
    list.add(7);
    list.add(65);
    list.add(67);

    list.findItem(65).expect("FIND ERROR: ");
    list.removeItem(7).expect("REMOVE ERROR: ");
    println!("{:#?}", list);
}
