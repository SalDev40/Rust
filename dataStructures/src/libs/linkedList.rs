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
    references: usize,
}

impl<T> Node<T> {
    fn new(data: T) -> Rc<RefCell<Node<T>>> {
        return Rc::new(RefCell::new(Node {
            data,
            pNext: None,
            references: 0,
        }));
    }
}

#[derive(Debug)]
struct LinkedList<T>
where
    T: Copy,
{
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    length: u64,
    itrPtr: Option<Rc<RefCell<Node<T>>>>,
}

/* Implement iterator for linkedList */
impl<T: Copy> Iterator for LinkedList<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let curNode;
        match &self.itrPtr {
            Some(node) => {
                //store reference to current node in list
                curNode = Rc::clone(&node);

                //move itrPtr to next node in list
                self.itrPtr = match &curNode.borrow().pNext {
                    Some(node) => Some(Rc::clone(&node)),
                    None => None,
                };

                return Some(curNode.borrow().data);
            }
            None => {
                //set itrPtr back to head node
                self.itrPtr = Some(Rc::clone(self.head.as_ref().unwrap()));
                return None;
            }
        }
    }
}

/* impl DataStucuture trait for linkedList */
impl<T: Debug + PartialEq + Copy> DataStructure<T> for LinkedList<T> {
    fn new() -> Self {
        return LinkedList {
            head: None,
            tail: None,
            length: 0,
            itrPtr: None,
        };
    }
    fn add(&mut self, data: T) {
        let newNode: Rc<RefCell<Node<T>>> = Node::new(data);
        match &mut self.head {
            Some(_) => {
                //add new Node to pNext ptr of tail
                self.tail
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .pNext
                    .replace(Rc::clone(&newNode));
            }
            None => {
                self.head = Some(Rc::clone(&newNode));
                self.itrPtr = Some(Rc::clone(&newNode));
            }
        }

        //store number of references made to this node
        newNode.borrow_mut().references = Rc::strong_count(&newNode);
        self.length += 1;
        self.tail = Some(newNode);
    }

    fn findItem(&mut self, item: T) -> Result<u64, Box<dyn std::error::Error>> {
        let mut i: u64 = 0;
        let mut found: bool = false;
        while i <= self.length && !found {
            match self.next() {
                Some(data) => {
                    if data == item {
                        found = true;
                        break;
                    }
                }
                None => break,
            };
            i += 1;
        }
        //reset itrPtr pointer back to head of list
        self.itrPtr = Some(Rc::clone(self.head.as_ref().unwrap()));
        if found {
            println!("item found at position -> {}", i);
            return Ok(i);
        }
        return Err(Box::new(Error::new(
            ErrorKind::Other,
            "find() -> ITEM NOT FOUND",
        )));
    }
    fn remove(&mut self, item: T) -> Result<bool, Box<dyn std::error::Error>> {
        let mut position: u64 = self.findItem(item)?;
        let mut i: u64 = 0;

        // create removePointer starting at head
        let mut pNodeBeforeRemove: Option<Rc<RefCell<Node<T>>>> =
            Some(Rc::clone(self.head.as_ref().unwrap()));

        //remove head node
        if position == 0 {
            println!("removing head node -> {:?}", item);
            let secondNode = Some(Rc::clone(
                &self.head.as_ref().unwrap().borrow().pNext.as_ref().unwrap(),
            ));
            self.head = secondNode;
            self.itrPtr = Some(Rc::clone(&self.head.as_ref().unwrap()));
            return Ok(true);
        };

        //traverse list to node before one to remove
        position -= 1;
        while i != position {
            pNodeBeforeRemove = Some(Rc::clone(
                &pNodeBeforeRemove
                    .unwrap()
                    .borrow_mut()
                    .pNext
                    .as_ref()
                    .unwrap(),
            ));
            i += 1;
        }

        //remove end node
        position += 1;
        if position == (self.length - 1) {
            println!("removing last node ->  {:?}", item);
            pNodeBeforeRemove.as_ref().unwrap().borrow_mut().pNext = None;
            self.tail = Some(Rc::clone(&pNodeBeforeRemove.as_ref().unwrap()));
            return Ok(true);
        }

        //remove middle nodes
        match &mut pNodeBeforeRemove {
            Some(node) => {
                println!("removing middle node ->  {:?}", item);
                //make node before the one to remove
                //point to node after the one to remove
                let nextNode = Some(Rc::clone(
                    node.borrow()
                        .pNext
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .pNext
                        .as_ref()
                        .unwrap(),
                ));
                node.borrow_mut().pNext = nextNode;
                return Ok(true);
            }
            _ => {}
        }

        //no nodes removed
        return Err(Box::new(Error::new(
            ErrorKind::Other,
            "remove() -> ITEM NOT Removed",
        )));
    }
}

pub fn linkMain() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.add(5);
    list.add(6);
    list.add(9);
    list.add(8);
    let removeNum = 9;
    match list.remove(removeNum) {
        Ok(status) => println!("{}: removed -> {}", status, removeNum),
        Err(e) => println!("{}", e),
    };
    // println!("list: {:#?}", list.head);
    // println!("list: {:#?}", list.tail);
}



impl<T: Debug + PartialEq + Copy> LinkedList<T> {
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
        //last element in head list also changes
        //ONLY change the pointer of the refNode which is consumed
        //by tail, since it is being referenced by the end Node in
        //the real list, any changes to its pNext ptr also changes
        //all other references
        match &self.tail {
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
                self.itrPtr = Some(Rc::clone(&newNode));
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

        newNode
            .borrow_mut() //get the value inside RefCell
            .references = Rc::strong_count(&newNode);

        self.tail = Some(newNode);
        println!("------------- Current List -----------------");
        // println!("{:#?}", self);
    }
}
