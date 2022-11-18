mod sudoku;

pub use sudoku::Sudoku;

fn main() {
    let mut s = Sudoku::blank();
    for i in 0..10 {
        println!("{i}");
        s.fill();
        s.pretty_print();
        println!("{i}");
    }
    
}