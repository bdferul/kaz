use super::Sudoku;
use termion::clear::CurrentLine;

impl Sudoku {
    pub fn raw_print(&self) {
        for y in 0..9 {
            for x in 0..9 {
                print!("{} ", self.raw_puzzle()[Self::crd(x, y)])
            }
            println!();
        }
    }

    pub fn pretty_print(&self) {
        let mut lines = vec![];
        let mut cps = vec![0;9];
        for y in 0..9 {
            let mut line = String::from("| ");
            for x in 0..9 {
                cps[x] = line.len();
                let n = self.raw_puzzle()[Self::crd(x,y)];
                line = format!("{}{}{}", 
                    line,
                    if n != 0 { n.to_string() } else { "_".to_string() }, 
                    if x == 2 || x == 5 { " | " } else { " " }
                );            
            }
            lines.push(format!("{line}|"));
        }

        let bar = vec!['-'; lines[0].len()].into_iter().collect::<String>();

        println!("{bar}");
        for y in 0..lines.len() {
            println!("{}", lines[y]);
            if y == 2 || y == 5 {
                println!("{bar}")
            }
        }
        println!("{bar}");
    }
}