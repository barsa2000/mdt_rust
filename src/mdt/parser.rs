#![allow(clippy::upper_case_acronyms)]

use crate::mdtrule::*;

use regex::Regex;
use std::mem::swap;

use pest::{iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "mdt/mdt.pest"]
struct MDT;

#[derive(Debug, Clone, Copy)]
pub enum BracesType {
    None,
    Square,
    Curly,
}

pub fn parse(source: &str) -> Vec<MDTRule> {
    let uppercase_source = source.to_uppercase();

    let mut rule_pairs =
        MDT::parse(Rule::rules, &uppercase_source).unwrap_or_else(|e| panic!("{}", e));
    generate_mdtrules_from_program(rule_pairs.next().unwrap())
}

fn generate_mdtrules_from_program(rules: Pair<Rule>) -> Vec<MDTRule> {
    assert!(rules.as_rule() == Rule::rules);
    let program = rules.into_inner();

    program
        .filter(|pair| pair.as_rule() == Rule::rule)
        .enumerate()
        .map(|(_, rule)| {
            generate_mdtrule_from_rule(rule)
            
        })
        .flatten()
        .collect()
}

fn generate_mdtrule_from_rule(rule: Pair<Rule>) -> Vec<MDTRule> {
    let mut rule_inner = rule.into_inner();

    let current_state_pair = rule_inner.next().unwrap();
    let read_symbol_pair = rule_inner.next().unwrap();
    let next_state_pair = rule_inner.next().unwrap();
    let write_symbol_pair = rule_inner.next().unwrap();
    let direction_pair = rule_inner.next().unwrap();

    let (current_b, current_sym, current_l) = generate_states_from_state_pair(current_state_pair);
    let (read_b, read_sta, read_l) = generate_states_from_symbol_pair(read_symbol_pair);
    let (next_b, next_sta, next_l) = generate_states_from_state_pair(next_state_pair);
    let (write_b, write_sym, write_l) = generate_states_from_symbol_pair(write_symbol_pair);
    let (dir_b, dir_dir, dir_l) = generate_states_from_direction_pair(direction_pair);

    let mut no_braces_len = None;
    let mut square_braces_len = None;
    let mut curly_braces_len = None;

    let mut update_lens = |brace_type: BracesType, len: usize| match brace_type {
        BracesType::None => {
            if len == 1 {
                return;
            }

            if no_braces_len.is_none() {
                no_braces_len = Some(len);
                return;
            }
            if no_braces_len.unwrap() != len {
                panic!(
                    "string too long: expected {}, found {}",
                    no_braces_len.unwrap(),
                    len
                );
            }
        }
        BracesType::Square => {
            if square_braces_len.is_none() {
                square_braces_len = Some(len);
                return;
            }
            if square_braces_len.unwrap() != len {
                panic!(
                    "string too long: expected {}, found {}",
                    square_braces_len.unwrap(),
                    len
                );
            }
        }
        BracesType::Curly => {
            if curly_braces_len.is_none() {
                curly_braces_len = Some(len);
                return;
            }
            if curly_braces_len.unwrap() != len {
                panic!(
                    "string too long: expected {}, found {}",
                    curly_braces_len.unwrap(),
                    len
                );
            }
        }
    };

    update_lens(current_b, current_l);
    update_lens(read_b, read_l);
    update_lens(next_b, next_l);
    update_lens(write_b, write_l);
    update_lens(dir_b, dir_l);

    let mut out = vec![];

    let get_index = |b: BracesType, nb: usize, sb: usize, cb: usize| match b {
        BracesType::None => nb,
        BracesType::Square => sb,
        BracesType::Curly => cb,
    };

    for nb in 0..no_braces_len.unwrap_or(1) {
        for sb in 0..square_braces_len.unwrap_or(1) {
            for cb in 0..curly_braces_len.unwrap_or(1) {
                out.push(MDTRule {
                    current_state: current_sym
                        .get(get_index(current_b, nb, sb, cb))
                        .unwrap_or_else(|| current_sym.last().unwrap())
                        .clone(),
                    read_symbol: *read_sta
                        .get(get_index(read_b, nb, sb, cb))
                        .unwrap_or_else(|| read_sta.last().unwrap()),
                    next_state: next_sta
                        .get(get_index(next_b, nb, sb, cb))
                        .unwrap_or_else(|| next_sta.last().unwrap())
                        .clone(),
                    write_symbol: *write_sym
                        .get(get_index(write_b, nb, sb, cb))
                        .unwrap_or_else(|| write_sym.last().unwrap()),
                    direction: *dir_dir
                        .get(get_index(dir_b, nb, sb, cb))
                        .unwrap_or_else(|| dir_dir.last().unwrap()),
                })
            }
        }
    }

    out
}

fn generate_states_from_state_pair(state_pair: Pair<Rule>) -> (BracesType, Vec<String>, usize) {
    let mut pair_inner = state_pair.into_inner();
    let mut maybe_pair = pair_inner.next();

    let mut first_string = "".to_string();
    if maybe_pair.is_some() {
        let pair = maybe_pair.clone().unwrap();
        if pair.as_rule() == Rule::string {
            first_string = cleanup_string(pair.as_str());
            maybe_pair = pair_inner.next();
        }
    }

    if maybe_pair.is_none() {
        return (BracesType::None, vec![first_string], 1);
    }

    let pair = maybe_pair.unwrap();

    let braces = match pair.as_rule() {
        Rule::square_br_list => BracesType::Square,
        Rule::curly_br_list => BracesType::Curly,
        _ => unreachable!(),
    };

    let mut list_inner = pair.into_inner();

    let compl_or_list = list_inner.next().unwrap();
    let list;
    let compl;
    if compl_or_list.as_rule() == Rule::compl {
        compl = true;
        list = list_inner.next().unwrap();
    } else {
        compl = false;
        list = compl_or_list;
    }

    let symbol_list: String = cleanup_string(&expand_symbol_list(list.as_str(), compl));

    let symbols_in_list = symbol_list.chars();

    maybe_pair = pair_inner.next();

    let second_string: String;

    match maybe_pair {
        Some(pair) => second_string = cleanup_string(pair.as_str()),
        None => second_string = "".to_string(),
    };

    let states_list: Vec<String> = symbols_in_list
        .map(|c| format!("{}{}{}", first_string, c, second_string))
        .collect();

    (braces, states_list.clone(), states_list.len())
}

fn generate_states_from_symbol_pair(pair: Pair<Rule>) -> (BracesType, Vec<char>, usize) {
    let braces = match pair.as_rule() {
        Rule::no_br_list => BracesType::None,
        Rule::compl => BracesType::None,
        Rule::square_br_list => BracesType::Square,
        Rule::curly_br_list => BracesType::Curly,
        _ => unreachable!(),
    };

    if pair.as_rule() != Rule::compl {
        let mut pair_inner = pair.into_inner();
        let compl_or_symbol_list = pair_inner.next().unwrap();
        let symbol_list;
        let compl;
        if compl_or_symbol_list.as_rule() == Rule::compl {
            compl = true;
            symbol_list = pair_inner.next().unwrap();
        } else {
            compl = false;
            symbol_list = compl_or_symbol_list;
        }

        let symbols_list: String = expand_symbol_list(symbol_list.as_str(), compl);

        let symbols_list = cleanup_string(symbols_list.as_str());
        (braces, symbols_list.chars().collect(), symbols_list.len())
    } else {
        let symbols_list: String = expand_symbol_list("", true);
        let symbols_list = cleanup_string(symbols_list.as_str());
        (braces, symbols_list.chars().collect(), symbols_list.len())
    }
}

fn generate_states_from_direction_pair(
    pair: Pair<Rule>,
) -> (BracesType, Vec<HeadDirection>, usize) {
    let braces = match pair.as_rule() {
        Rule::no_br_list_d => BracesType::None,
        Rule::square_br_list_d => BracesType::Square,
        Rule::curly_br_list_d => BracesType::Curly,
        _ => unreachable!(),
    };

    let directions_list = pair
        .as_str()
        .chars()
        .map(|c| match c {
            '>' => HeadDirection::Right,
            '<' => HeadDirection::Left,
            '-' => HeadDirection::None,
            _ => unreachable!(),
        })
        .collect();

    (braces, directions_list, pair.as_str().len())
}

fn cleanup_string(s: &str) -> String {
    lazy_static! {
        static ref SPACES_REGEX: Regex = Regex::new(r"(?P<before>[^\\]?)\-").unwrap();
        static ref SPECIAL_CHARS_REGEX: Regex = Regex::new(r"\\(?P<next>.)").unwrap();
    }
    let out = SPACES_REGEX.replace_all(s, "${before} ").to_string();
    let out = SPECIAL_CHARS_REGEX.replace_all(&out, "${next}").to_string();

    out
}

fn expand_symbol_list(symbol_list: &str, compl: bool) -> String {
    const ALPHABET: &str = " !\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^-{|}";

    let split_list = symbol_list.split("..");

    let expanded = split_list
        .zip(symbol_list.split("..").skip(1))
        .map(|(c, n)| {
            let current_last = c.chars().last().unwrap();
            let next_first = n.chars().next().unwrap();

            let mut filling: Vec<char> = vec![];
            if current_last.is_numeric() && next_first.is_numeric()
                || current_last.is_alphabetic() && next_first.is_alphabetic()
            {
                let mut a = current_last;
                let mut b = next_first;
                let swapped = a > b;
                if a > b {
                    swap(&mut a, &mut b);
                }
                (a..b).skip(1).for_each(|c| filling.push(c));
                if swapped {
                    filling.reverse();
                }
            } else {
                panic!("range sbagliato")
            }

            format!("{}{}", c, filling.iter().collect::<String>())
        })
        .collect::<String>()
        + symbol_list.split("..").last().unwrap();

    if compl {
        ALPHABET.replace(&expanded.chars().collect::<Vec<char>>()[..], "")
    } else {
        expanded
    }
}
