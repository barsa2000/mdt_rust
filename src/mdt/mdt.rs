use std::cell::RefCell;
use std::fmt::{self};
use std::rc::{Rc, Weak};

use crate::mdtrule::*;

#[derive(Debug)]
pub struct Node {
    id: i64,
    value: char,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn add_next(node: &mut Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        Self::add_next_with_value(node, ' ')
    }

    fn add_next_with_value(node: &mut Rc<RefCell<Node>>, c: char) -> Rc<RefCell<Node>> {
        let new = Node {
            id: node.borrow().id + 1,
            value: c,
            next: None,
            prev: Some(Rc::downgrade(node)),
        };
        let r = Rc::new(RefCell::new(new));
        node.borrow_mut().next = Some(r.clone());
        r
    }

    fn add_prev(node: &mut Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        Self::add_prev_with_value(node, ' ')
    }

    fn add_prev_with_value(node: &mut Rc<RefCell<Node>>, c: char) -> Rc<RefCell<Node>> {
        let new = Node {
            id: node.borrow().id - 1,
            value: c,
            next: Some(node.clone()),
            prev: None,
        };
        let r = Rc::new(RefCell::new(new));
        node.borrow_mut().prev = Some(Rc::downgrade(&r));
        r
    }
}

pub struct Mdt {
    rules: Vec<MDTRule>,
    first: Rc<RefCell<Node>>,
    head: Rc<RefCell<Node>>,
    state: String,
    id_offset: u32,
}

impl Mdt {
    pub fn new(initial_tape: &str, rules: Vec<MDTRule>) -> Self {
        let rules = rules.iter().rev().cloned().collect();
        let first = Self::load_tape(initial_tape);
        Self {
            rules,
            first: first.clone(),
            head: first,
            state: "0".to_string(),
            id_offset: 0,
        }
    }

    pub fn step(&mut self) -> bool {
        let maybe_rule = self
            .rules
            .iter()
            .find(|r| r.current_state == self.state && r.read_symbol == self.get_head_value());
        if let Some(rule) = maybe_rule {
            self.set_head_value(rule.write_symbol);
            self.state = rule.next_state.clone();
            self.move_head(rule.direction);
            true
        } else {
            false
        }
    }

    fn load_tape(initial_tape: &str) -> Rc<RefCell<Node>> {
        let first = Rc::new(RefCell::new(Node {
            id: 0,
            value: ' ',
            next: None,
            prev: None,
        }));
        if !initial_tape.is_empty() {
            let mut chars = initial_tape.chars();
            first.borrow_mut().value = chars.next().unwrap();

            let mut cur = first.clone();
            for c in chars {
                cur = Node::add_next_with_value(&mut cur, c);
            }
        };
        first
    }

    fn get_head_value(&self) -> char {
        self.head.borrow().value
    }

    fn set_head_value(&self, value: char) {
        self.head.borrow_mut().value = value;
    }

    fn move_head(&mut self, dir: HeadDirection) {
        match dir {
            HeadDirection::Right => {
                if self.head.borrow_mut().next.is_none() {
                    Node::add_next(&mut self.head);
                }
                self.head = self.head.clone().borrow().next.clone().unwrap();
            }
            HeadDirection::Left => {
                if self.head.borrow_mut().prev.is_none() {
                    self.first = Node::add_prev(&mut self.head);
                    self.id_offset += 1;
                }
                self.head = self
                    .head
                    .clone()
                    .borrow()
                    .prev
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap();
            }
            HeadDirection::None => {}
        }
    }

    pub fn as_char_vec(&self) -> Vec<char> {
        let mut cur = self.first.clone();
        let mut out = vec![];
        loop {
            out.push(cur.borrow().value);
            if let Some(new_head) = cur.clone().borrow().next.clone() {
                cur = new_head;
            } else {
                break;
            }
        }
        out
    }
}

impl fmt::Display for Mdt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = self.as_char_vec().iter().collect::<String>();
        write!(f, "{}", v.trim())
    }
}

impl fmt::Debug for Mdt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cur = self.first.clone();
        let mut out = "".to_string();
        let mut head_pos = 0;
        loop {
            if Rc::ptr_eq(&self.head, &cur) {
                out += &format!(">'{}'<,", cur.borrow().value);
                head_pos = cur.borrow().id;
            } else {
                out += &format!("'{}',", cur.borrow().value);
            }

            if let Some(new_head) = cur.clone().borrow().next.clone() {
                cur = new_head;
            } else {
                out.pop();
                break;
            }
        }

        write!(
            f,
            "[{}] -- head pos id: {}, state: {}",
            out, head_pos, self.state
        )
    }
}
