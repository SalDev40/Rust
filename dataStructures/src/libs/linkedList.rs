#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
use crate::libs::common::DataStructure;
use std::cell::RefCell;
use std::fmt::Debug;
use std::io::{Error, ErrorKind};
use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
    data: T,
    pNext: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Rc<RefCell<Node<T>>> {
        return Rc::new(RefCell::new(Node { data, pNext: None }));
    }
}

#[derive(Debug, Clone)]
struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    length: u64,
    current: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone + Debug + Copy> Iterator for LinkedList<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let curNode;
        match &self.current {
            Some(node) => {
                curNode = node.clone();
                // make current = to next node by copying
                //the option value of next node into it
                //if their is no next Node -> end of list, return None`
                self.current = match &curNode.borrow().pNext {
                    Some(node) => Some(Rc::clone(&node)),
                    None => None,
                };
                return Some(curNode.borrow().data);
            }
            None => {
                //set current back to head node
                self.current = Some(Rc::clone(self.head.as_ref().unwrap()));
                return None;
            }
        }
    }
}

impl<T: Debug + PartialEq> LinkedList<T> {
    pub fn addDiff2(&mut self, data: T) {
        let newNode: Rc<RefCell<Node<T>>> = Node::new(data);
        println!("-----------------");
        println!(
            "Creating a newNode {:?}, References ==> {:?}",
            newNode,
            Rc::strong_count(&newNode)
        );

        //since the tail node is shared reference with last node in list
        //starting from head, when we change its pNext pointer  the
        //last element in head also changes
        match self.tail.take() {
            Some(oldLast) => {
                println!(
                    "Shared references to oldNode {:?}: {:?}",
                    oldLast.borrow().data,
                    Rc::strong_count(&oldLast)
                );
                println!(
                    "Pointing oldNode.pNext {:?} -> {:?}",
                    oldLast.borrow().pNext,
                    newNode
                );
                oldLast.borrow_mut().pNext = Some(Rc::clone(&newNode));
                println!("Creating a shared Reference to newNode from oldNode.pNext");
            }
            None => {
                println!("Adding to head {:?}", newNode);
                //will be a shared reference to the last tail node initially
                //pNext time around when tail adds a new node, since this
                //is shared it adds a new node to its list.
                self.head = Some(Rc::clone(&newNode));
                self.current = Some(Rc::clone(&newNode));
                println!("Creating a shared Reference to newNode");
            }
        }
        self.length += 1;

        println!(
            "Shared references to newNode {:?} ==> {:?}",
            newNode,
            Rc::strong_count(&newNode)
        );
        println!("Changing tail to  consume {:?}", newNode);

        //this keeps referece to last node just added
        self.tail = Some(newNode);
        println!("------------- Current List -----------------");
        // println!("{:#?}", self);
    }
}

impl<T: Debug + PartialEq> DataStructure<T> for LinkedList<T> {
    fn new() -> Self {
        return LinkedList {
            head: None,
            tail: None,
            length: 0,
            current: None,
        };
    }
    fn add(&mut self, data: T) {
        let newNode: Rc<RefCell<Node<T>>> = Node::new(data);
        println!("-----------------");
        // println!(
        //     "Creating a newNode {:?}, References ==> {:?}",
        //     newNode,
        //     Rc::strong_count(&newNode)
        // );

        match &mut self.head {
            Some(_) => {
                // println!("{:?}", self.tail.as_ref().unwrap());
                self.tail
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .pNext
                    .replace(Rc::clone(&newNode));
                ()
            }
            None => {
                self.head = Some(Rc::clone(&newNode));
                self.current = Some(Rc::clone(&newNode));
            }
        }
        self.tail = Some(newNode);
        // println!("------------- Current List -----------------");
        // println!("{:#?}", self);
    }
    fn find(&self, item: T) -> Result<T, Box<dyn std::error::Error>> {
        match &self.head {
            Some(node) => Ok(item),
            None => Err(Box::new(Error::new(ErrorKind::Other, "List is Empty!!"))),
        }
    }
    fn remove(&self, item: T) -> bool {
        return true;
    }
}

pub fn linkMain() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.add(5);
    list.add(6);

    println!("{:?}", list.next().expect("List empty"));
    println!("{:?}", list.next().expect("List empty"));
    println!("{:?}", list.next().expect("List empty"));
    // list.add(9);
    // list.add(12);
    // list.add(13);
    // list.add(14);
    // list.add(4);
    // list.addDiff2(5);
    // list.addDiff2(6);
    // list.addDiff2(9);
}
