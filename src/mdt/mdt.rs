use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{self};
use std::rc::{Rc, Weak};

use crate::mdtrule::*;
use crate::parser;

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
    rules: HashMap<char, HashMap<String, MDTRule>>,
    first: Rc<RefCell<Node>>,
    head: Rc<RefCell<Node>>,
    state: String,
    id_offset: u32,
}

impl Mdt {
    pub fn new(initial_tape: &str, rules_str: &str) -> Self {
        let mut rules = HashMap::new();
        let rules_vec = parser::parse(rules_str);

        println!("rules in vec: {}", rules_vec.len());

        rules_vec.iter().for_each(|r| {
            let a = rules.entry(r.read_symbol).or_insert_with(HashMap::new);
            a.insert(r.current_state.clone(), r.clone());
        });
        let hm_len = rules.iter().fold(0, |sum,(_,s_hm)|{sum + s_hm.len()});
        println!("rules in hm: {}", hm_len);

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
        let maybe_sym_hashmap = self.rules.get(&self.get_head_value());
        if let Some(sym_hashmap) = maybe_sym_hashmap {
            let maybe_rule = sym_hashmap.get(&self.state);
            if let Some(r) = maybe_rule {
                self.set_head_value(r.write_symbol);
                self.state = r.next_state.clone();
                self.move_head(r.direction);
                return true;
            }
        }
        false
    }

    fn load_tape(initial_tape: &str) -> Rc<RefCell<Node>> {
        let initial_tape = initial_tape.to_uppercase();
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
