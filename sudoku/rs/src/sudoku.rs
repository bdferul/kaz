use rand::{thread_rng, seq::SliceRandom};

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
            let crd = Self::crd(0,y);
            v2d.push(self.puz[crd..crd+9].to_vec());
        }
        v2d
    }

    pub fn raw_puzzle(&self) -> P {
        self.puz.clone()
    }

    fn get_possibilities(&self, pos: usize) -> Vec<u8> {
        let mut bad = vec![];
        let (px,py) = Sudoku::fcrd(pos);

        //horizontal
        for x in 0..8 {
            if x >= px {
                break;
            }
            bad.push(self.puz[Self::crd(x, py)]);
        }

        //vertical
        for y in 0..8 {
            if y >= py {
                break;
            }
            bad.push(self.puz[Self::crd(px, y)]);
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

                bad.push(self.puz[Self::crd(cx, cy)]);
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

    pub fn crd(x: usize, y: usize) -> usize {
        x + (y * 9)
    }

    /// Returns coordinate pair from usize
    pub fn fcrd(pos: usize) -> (usize,usize) {
        (pos%9, pos/9)
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

                    println!("{}: {:?}", line!(), (pos, bv, bi));
                }
            }

            pos += 1;
            gens += 1;
        }
        println!("gens: {gens}");
    }
}