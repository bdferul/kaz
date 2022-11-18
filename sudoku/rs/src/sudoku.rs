use std::{env::current_exe, io::stdout};

use rand::{thread_rng, seq::SliceRandom};
use termion::clear::CurrentLine;

type P = Vec<u8>;

pub struct Sudoku {
    puz: P,
}

impl Sudoku {
    pub fn blank() -> Sudoku {
        Sudoku {
            puz: vec![0;81],
        }
    }

    pub fn v2d(&self) -> Vec<Vec<u8>> {
        let mut v2d = vec![];
        for y in 0..9 {
            let crd = crd(0,y);
            v2d.push(self.puz[crd..crd+9].to_vec());
        }
        v2d
    }

    pub fn raw_puzzle(&self) -> P {
        self.puz.clone()
    }

    pub fn raw_print(&self) {
        for y in 0..9 {
            for x in 0..9 {
                print!("{} ", self.puz[crd(x, y)])
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
                let n = self.puz[crd(x,y)];
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

    fn get_possibilities(&self, pos: usize) -> Vec<u8> {
        let mut bad = vec![];
        let (px,py) = fcrd(pos);

        //horizontal
        for x in 0..8 {
            if x >= px {
                break;
            }
            bad.push(self.puz[crd(x, py)]);
        }

        //vertical
        for y in 0..8 {
            if y >= py {
                break;
            }
            bad.push(self.puz[crd(px, y)]);
        }

        //square
        let h = |x| (x/3)*3;
        let (sx,sy) = (h(px), h(py));
        for y in 0..3 {
            for x in 0..3 {
                let (cx,cy) = (x+sx,y+sy);
                if cx == px && cy == py {
                    break;
                }

                bad.push(self.puz[crd(cx, cy)]);
            }
        }

        let mut good = vec![];
        for i in 1..=9 {
            if !bad.contains(&i) {
                good.push(i);
            }
        }

        good
    }

    /// Fills the puz member with a valid sudoku puzzle
    pub fn fill(&mut self) {
        let mut rng = thread_rng();

        let mut pos = 0;
        let mut gps = vec![(vec![],0);81];
        
        let mut gens = 0;

        while pos < self.puz.len() {
            let mut v = self.get_possibilities(pos);
            v.shuffle(&mut rng);
            

            if v.len() > 0 {
                self.puz[pos] = v[0];
                gps[pos] = (v,0);
            } else {
                let mut backtrace = true;
                while backtrace {
                    pos -= 1;

                    let (bv,bi) = &mut gps[pos];
                    *bi += 1;

                    if *bi < bv.len() {
                        self.puz[pos] = bv[*bi];
                        backtrace = false;
                    }
                }
            }
            print!("{}\r{} {}", CurrentLine, gens, pos);

            pos += 1;
            gens += 1;
        }
        println!("gens: {gens}");
    }
}

/// Returns usize from coordinate pair
fn crd(x: usize, y: usize) -> usize {
    x + (y * 9)
}

/// Returns coordinate pair from usize
fn fcrd(pos: usize) -> (usize,usize) {
    (pos%9, pos/9)
}