use std::{rc::Rc, cell::RefCell, borrow::{Borrow, BorrowMut}, num, cmp::Ordering};
use core::borrow;

use crate::{Solution, SolutionPair, etc::utils};

///////////////////////////////////////////////////////////////////////////////

struct Element {
    value: Option<u32>,
    children: Vec<Rc<RefCell<Element>>>,
    parent: Option<Rc<RefCell<Element>>>,
}

impl Element {
    pub fn new() -> Element {
        return Element {
            value: None,
            children: vec![],
            parent: None,
        };
    }

    pub fn add_child(&mut self, new_node: Rc<RefCell<Element>>) {
        // self.children.push(new_node);
        // let child = new_node.borrow_mut();
        // child.parent = Some(Rc::clone(self));
    }

    pub fn print(&self) -> String {
        if let Some(value) = self.value {
        return value.to_string();
        } else {
        return String::from("[")
            + &self
            .children
            .iter()
            .map(|tn| tn
                .as_ref()
                .borrow()
                .print())
            .collect::<Vec<String>>()
            .join(",")
            + "]";
        }
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty() && self.value.is_none()
    }

    pub fn is_value(&self) -> bool {
        self.children.is_empty() && !self.value.is_none()
    }
    
    pub fn is_list(&self) -> bool {
        !self.children.is_empty() && self.value.is_none()
    }
    
    pub fn cmp(&mut self, other_ref: Rc<RefCell<Element>>) -> Ordering {
        let mut other = other_ref.as_ref().borrow_mut();
        if self.is_value() && other.is_list() {
            self.children.push(Rc::new(RefCell::new(Element {value: self.value, children: vec![], parent: None})));
            self.value = None;
            // return self.cmp(Rc::clone(&other_ref));
        }
        if self.is_list() && other.is_value() {
            let v = other.value;
            other.children.push(Rc::new(RefCell::new(Element {value: v, children: vec![], parent: None})));
            other.value = None;
            // return self.cmp(Rc::clone(&other_ref));
        }
        if self.is_value() && other.is_value() {
            return self.value.unwrap().cmp(&other.value.unwrap());
        }
        if self.is_list() && other.is_list() {
            let i1 = self.children.iter();
            let mut i2 = other.children.iter_mut();
            for e1 in i1 {
                let e2 = i2.next();
                if e2.is_none() {
                    return Ordering::Less;
                } else {
                    let o = e1.as_ref().borrow_mut().cmp(Rc::clone(&e2.unwrap()));
                    if o != Ordering::Equal {
                        return o;
                    }
                }
            }
            if i2.next().is_none() {
                return Ordering::Equal;
            } else {
                return Ordering::Greater;
            }
        }
        if self.is_empty() && !other.is_empty() { return Ordering::Less; }
        if !self.is_empty() && other.is_empty() { return Ordering::Greater; }
        Ordering::Equal
    }
}

pub fn solve() -> SolutionPair {

    let lines = utils::read_lines("./input/input_13");

    let mut root1 = Rc::new(RefCell::new(Element::new()));
    let mut root2 = Rc::new(RefCell::new(Element::new()));
    let mut current;

    for (i, line) in lines.iter().enumerate() {
        if i % 3 == 2 {continue;}
        else if i % 3 == 0 {
            root1 = Rc::new(RefCell::new(Element::new()));
            current = Rc::clone(&root1);
        } else {
            root2 = Rc::new(RefCell::new(Element::new()));
            current = Rc::clone(&root2);
        }
        let mut num_buff = Vec::new();
        for c in line.chars() {
            if c == '[' {
                let child = Rc::new(RefCell::new(Element::new()));
                current.as_ref().borrow_mut().children.push(Rc::clone(&child));
                {
                    let mut mut_child = child.as_ref().borrow_mut();
                    mut_child.parent = Some(Rc::clone(&current));
                }
                current = child;
            } else if c.is_numeric() {
                num_buff.push(c);
            } else if c == ',' || c == ']' {
                if !num_buff.is_empty() {
                    let child = Rc::new(RefCell::new(Element::new()));
                    current.as_ref().borrow_mut().children.push(Rc::clone(&child));
                    {
                        let mut mut_child = child.as_ref().borrow_mut();
                        mut_child.parent = Some(Rc::clone(&current));
                        mut_child.value = Some(num_buff.iter().collect::<String>().parse::<u32>().unwrap());
                    }
                    current = child;
                    num_buff.clear();
                }

                let current_clone = Rc::clone(&current);
                current = Rc::clone(current_clone.as_ref().borrow().parent.as_ref().unwrap());
            } else {
                panic!("Unknown character: {}", c);
            }
        }
        println!("{:?}", current.as_ref().borrow().print());
        if i % 3 == 1 {
            println!("{:?}", root1.as_ref().borrow_mut().cmp(Rc::clone(&root2)));
        }
    }

    // println!("{}", root1.as_ref().borrow().print());
    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::U64(sol1), Solution::U64(sol2))
}
