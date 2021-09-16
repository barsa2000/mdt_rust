use mdt::parser;
use mdt::Mdt::Mdt;

// bin replace
// static ISTRUZIONI: &str = "
// (0,[01],0,[10],>)
// (0,-,fine,-,-)";

// sette e mezzo
static ISTRUZIONI: &str = "
(0,[1..7FCR],[1..7AAA],-,>)

([1..7],{FCR},[1..7]A,-,>)
(A,[1..7],[1..7]A,-,>)

(1,[1..6],[2..7],-,>)
(1,7,B,-,>)
(1A,[1..6],[2..7]A,-,>)
(1A,7,B,-,>)

(2,[1..5],[3..7],-,>)
(2,[67],B,-,>)
(2A,[1..5],[3..7]A,-,>)
(2A,[67],B,-,>)

(3,[1..4],[4..7],-,>)
(3,[567],B,-,>)
(3A,[1..4],[4..7]A,-,>)
(3A,[567],B,-,>)

(4,[1..3],[5..7],-,>)
(4,[4567],B,-,>)
(4A,[1..3],[5..7]A,-,>)
(4A,[4567],B,-,>)

(5,[12],[67],-,>)
(5,[3..7],B,-,>)
(5A,[12],[67]A,-,>)
(5A,[3..7],B,-,>)

(6,1,7,-,>)
(6,[2..7],B,-,>)
(6A,1,7A,-,>)
(6A,[2..7],B,-,>)

(7,[1..7],B,-,>)
(7A,[1..7],B,-,>)

([1..7]A,-,OK,O,>)
([1..7]A,{FRC},[2..7B],-,>)
([1..7],-,OK,O,>)

(B,^-,B,-,>)
(B,-,KO,K,>)

(OK,-,FINE,K,-)
(KO,-,FINE,O,-)";

static NASTRO: &str = "3R4";

fn main() {
    let input = ISTRUZIONI;

    let rules = parser::parse(input);

    // println!("{}", input);

    println!("rules count: {}", rules.len());
    // rules.iter().for_each(|r| println!("{}", r));

    let mut mdt = Mdt::new(NASTRO, rules);
    println!("{:#?}", mdt);

    loop {
        let step_res = mdt.step();
        println!("{:#?}", mdt);
        if !step_res {
            break;
        }
    }

    let out = mdt.to_string();

    println!("input string:  {}", NASTRO);
    println!("output string: {}", out);
}
