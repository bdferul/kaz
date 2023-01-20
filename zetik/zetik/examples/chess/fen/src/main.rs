use zetik::Chess;

fn main() {
    let d = Chess::default();
    println!("default: {}", d.to_fen());
    println!("url: {}", d.to_fen_url());
    let s = "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50".to_string();
    let ff = Chess::from_fen(s.clone()).unwrap();
    println!("{s}");
    println!("{}", ff.to_fen());
    ff.pretty_print();
    println!("url: {}", ff.to_fen_url());
    //ff.pretty_print();
}
