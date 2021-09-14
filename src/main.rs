use mdt::parser;

// static TEXT: &str = "
// (0,[01],0,[10],>)";

static ISTRUZIONI: &str = "
({0},[01],{0},[10],>)
(0,-,fine,-,-)";

static NASTRO: &str = "0101010101";

fn main() {
    let input = ISTRUZIONI;

    let rules = parser::parse(input);

    println!("{}", input);

    println!("rules: {:#?}", rules);
}
