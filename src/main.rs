use std::time::Instant;

use mdt::mdt::Mdt;

// bin replace
// static ISTRUZIONI: &str = "
// (0,[01],0,[10],>)
// (0,-,fine,-,-)
// (0,0,0,5,>)";
// static NASTRO: &str = "111r1111";

// sette e mezzo
// static ISTRUZIONI: &str = "
// (0,[1..7FCR],[1..7AAA],-,>)

// ([1..7],{FCR},[1..7]A,-,>)
// (A,[1..7],[1..7]A,-,>)

// (1,[1..6],[2..7],-,>)
// (1,7,B,-,>)
// (1A,[1..6],[2..7]A,-,>)
// (1A,7,B,-,>)

// (2,[1..5],[3..7],-,>)
// (2,[67],B,-,>)
// (2A,[1..5],[3..7]A,-,>)
// (2A,[67],B,-,>)

// (3,[1..4],[4..7],-,>)
// (3,[567],B,-,>)
// (3A,[1..4],[4..7]A,-,>)
// (3A,[567],B,-,>)

// (4,[1..3],[5..7],-,>)
// (4,[4567],B,-,>)
// (4A,[1..3],[5..7]A,-,>)
// (4A,[4567],B,-,>)

// (5,[12],[67],-,>)
// (5,[3..7],B,-,>)
// (5A,[12],[67]A,-,>)
// (5A,[3..7],B,-,>)

// (6,1,7,-,>)
// (6,[2..7],B,-,>)
// (6A,1,7A,-,>)
// (6A,[2..7],B,-,>)

// (7,[1..7],B,-,>)
// (7A,[1..7],B,-,>)

// ([1..7]A,-,OK,O,>)
// ([1..7]A,{FRC},[2..7B],-,>)
// ([1..7],-,OK,O,>)

// (B,^-,B,-,>)
// (B,-,KO,K,>)

// (OK,-,FINE,K,-)
// (KO,-,FINE,O,-)";
// static NASTRO: &str = "111r1111";

//la bomba
static ISTRUZIONI: &str = "
(0, [a..z], init, [a..z], <)

(init, -, init2, @, <)
(init2, -, goConv, !, >)

(ckRot, [!\"#$%&\'\\(\\)*+\\,-\\./0..8], goConv, [\"#$%&\'\\(\\)*+\\,-\\./0..9], >)
(ckRot, 9, NO, -, >)

(goConv, ^=, goConv, ^=, >)
(goConv, =, conv, =, >)

(conv, [a..z], conv, [b..za], >)
(conv, -, goCpy, @, <)
(conv, @, goCpy, @, <)

(goCpy, [a..z], goCpy, [a..z], <)
(goCpy, @, goCpy, @, <)
(goCpy, =, find, =, >)
(goCpy, [!\"#$%&\'\\(\\)*+\\,-\\./0123456789:], find, [!\"#$%&\'\\(\\)*+\\,-\\./0123456789:], >)

(find, [!\"#$%&\'\\(\\)*+\\,-\\./0123456789:], find, [!\"#$%&\'\\(\\)*+\\,-\\./0123456789:], >)
(find, [a..z], piazza[a..z], [!\"#$%&\'\\(\\)*+\\,-\\./0123456789:], >)
(find, @, goConf, @, <)

(piazza[a..z], ^-, piazza[a..z], ^-, >)
(piazza[a..z], -, goCpy, [a..z], <)

(goConf, [!\"#$%&\'\\(\\)*+\\,-\\./0123456789:], goConf, [a..z], <)
(goConf, =, goConf2, =, <)

(goConf2, ^@, goConf2, ^@, <)
(goConf2, @, conf, @, >)

(conf, [a..z], gogoCk1[a..z], [!\"#$%&\'\\(\\)*+\\,-\\./0123456789:], >)
(conf, [!\"#$%&\'\\(\\)*+\\,-\\./0123456789:], conf2, [!\"#$%&\'\\(\\)*+\\,-\\./0123456789:], >)

(conf2, [!\"#$%&\'\\(\\)*+\\,-\\./0123456789:], conf2, [!\"#$%&\'\\(\\)*+\\,-\\./0123456789:], >)
(conf2, [a..z], gogoCk2[a..z], [!\"#$%&\'\\(\\)*+\\,-\\./0123456789:], >)
(conf2, ?, SI, ?, >)

(gogoCk1[a..z], ^=, gogoCk1[a..z], ^=, >)
(gogoCk1[a..z], =, goCk1[a..z], =, >)

(goCk1[a..z], ^, NOSTR, \\^, <)
(goCk1[a..z], \\^, goCk1[a..z], \\^, >)
(goCk1[a..z], @, NOROT, @, -)
(goCk1[a..z], [a..z], go@, \\^, <)

(gogoCk2[a..z], ^=, gogoCk2[a..z], ^=, >)
(gogoCk2[a..z], =, goCk2[a..z], =, >)

(goCk2[a..z], ^, NOSTR, ^, <)
(goCk2[a..z], \\^, goCk2[a..z], \\^, >)
(goCk2[a..z], @, NOROT, @, -)
(goCk2[a..z], [a..z], go@, [!\"#$%&\'\\(\\)*+\\,-\\./0123456789:], <)
(goCk2[a..z], {!\"#$%&\'\\(\\)*+\\,-\\./0123456789:}, goCk2[a..z], {!\"#$%&\'\\(\\)*+\\,-\\./0123456789:}, >)

(go@, ^@, go@, ^@, <)
(go@, @, conf, @, >)

(NOSTR, [!\"#$%&\'\\(\\)*+\\,-\\./0123456789:], NOSTR, [a..z], <)
(NOSTR, [a..z], NOSTR, [a..z], <)
(NOSTR, [?=], NOSTR, [?=], <)
(NOSTR, \\^, NOSTR, \\^, <)
(NOSTR, @, conf, @, >)

(NOROT, @, piazzaSpeciale, @, <)

(piazzaSpeciale, \\^, goRicopia, \\^, >)
(piazzaSpeciale, [!\"#$%&\'\\(\\)*+\\,-\\./0123456789:], piazzaSpeciale, \\^, <)

(goRicopia, ^-, goRicopia, ^-, >)
(goRicopia, -, ck, -, <)

(ck, @, goIncRot, @, <)
(ck, [a..z], goWr[a..z], -, <)

(goWr[a..z], {a..z}, goWr[a..z], {a..z}, <)
(goWr[a..z], @, wr[a..z], @, <)

(wr[a..z], {a..z}, wr[a..z], {a..z}, <)
(wr[a..z], \\^, goRicopia, [a..z], >)

(goIncrot, [!\"#$%&\'\\(\\)*+\\,-\\./0123456789:], goIncRot, [a..z], <)
(goIncRot, ^@, goIncRot, ^@, <)
(goIncRot, @, ckRot, @, <)

(NO, ^-, NO, -, >)
(NO, -, fine, ?, -)

(SI, ^-, SI, -, <)
(SI, -, goStr, -, >)

(goStr, -, goStr, -, >)
(goStr, ^@, pz, ^@, -)

(pz, ^@, pz, \\^, >)
(pz, @, goRic, @, >)

(goRic, ^-, goRic, ^-, >)
(goRic, -, ckk, -, <)

(fine, ^-, fine, ^-, <)
(fine, -, ciao, -, >)

(ciao, \\^, ciao, -, >)

(ckk, @, fine, -, <)
(ckk, [a..z], goWrr[a..z], -, <)

(goWrr[a..z], {a..z}, goWrr[a..z], {a..z}, <)
(goWrr[a..z], @, wrr[a..z], @, <)

(wrr[a..z], {a..z}, wrr[a..z], {a..z}, <)
(wrr[a..z], \\^, goRic, [a..z], >)";
static NASTRO: &str = "alba?=dwwdffduhdoodoed";

fn main() {
    let input = ISTRUZIONI;

    let start = Instant::now();
    let mut mdt = Mdt::new(NASTRO, input);
    // println!("{:#?}", mdt);
    let mut it: u64 = 0;
    loop {
        let step_res = mdt.step();
        // println!("{:#?}", mdt);
        if !step_res || it > 1000000 {
            break;
        }
        it += 1;
    }
    let elapsed = start.elapsed().as_millis();

    let out = mdt.to_string();

    println!("input string:  {}", NASTRO);
    println!("output string: {}", out);
    println!("computations: {}", it);
    println!("took: {}ms", elapsed);
}
