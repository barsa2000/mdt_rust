#![allow(clippy::upper_case_acronyms)]

use pest::{iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "mdt_parser/mdt.pest"]
struct MDT;

#[derive(Debug)]
struct Program {
    //TODO: implement program
}

fn parse(source: &str) -> Program{
    todo!()
}