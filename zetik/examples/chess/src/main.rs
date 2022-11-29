use zetik::Chess;

fn main() {
    let mut d = Chess::default();
    println!("{:?}", d.mv(ndx(0,6),ndx(0,4)));
}

fn ndx(x:usize,y:usize) -> usize {
    x + (y*8)
}