use crate::{chess::{Chess, piece::*, Side::*, Class::*}, mdx, fmdx};

impl Chess {
        ///
        pub(super) fn complex_choices(&self, src: usize) -> Vec<usize> {
            let mut r = vec![];

            let Some(src_p) = self.board[src] else {
                return r
            };

            if src_p.class == King {
                r.extend(self.castle_choices(src_p.side));
            }
            
            r
        }

        pub(super) fn preform_en_passant(&mut self, src: usize, dst: usize) {
            let Some(ep) = self.en_passant else {
                return
            };
            let src_p = self.board[src].unwrap();
            if src_p.class != Pawn {
                return
            }
    
            let (_,src_y) = fmdx!(src);
            let (dst_x,_) = fmdx!(dst);
    
            if dst == ep {
                self.board[mdx!(dst_x,src_y)] = NO;
            }
        }
    
        /// Will panic if `self.board[src].is_none()`
        pub(super) fn set_en_passant(&mut self, src: usize, dst: usize) {
            let src_p = self.board[src].unwrap();
    
            if src_p.class != Pawn {
                self.en_passant = None;
            }
    
            let dir = if src < dst {1} else {-1};
    
            let (src_x,src_y) = fmdx!(src, i32);
            let (_,dst_y) = fmdx!(dst, i32);
    
            if matches!(dst_y, 3|4) && matches!(src_y, 1|6) {
                self.en_passant = Some(mdx!(src_x, src_y + dir));
            } else {
                self.en_passant = None;
            }
        }

        /// Returns positions available for castling
        fn castle_choices(&self, side: Side) -> Vec<usize> {
            let mut r = vec![];

            let srcs = [60,4]; // [w,b]
            let dsts = [62, 58, 6, 2]; // [K,Q,k,q]

            for (i,_) in self.castle.into_iter().enumerate().filter(|(_,x)| *x) {
                if (i < 2 && side != White) || (i >= 2 && side != Black) {
                    continue;
                }

                let src = srcs[i/2];
                let dst = dsts[i];

                let mid = if src < dst {src + 1} else {src - 1};

                if !(self.is_dangerous(src as usize) || self.is_dangerous(mid) || self.board[mid].is_some() || self.board[dst].is_some()) {
                    r.push(dst);
                }
            }
            
            r
        }

        pub(super) fn perform_castle(&mut self, src: usize, dst: usize) {
            if self.board[src].unwrap().class != King {
                return
            }

            if matches!(src, 4 | 60) {
                let a = src - 2;
                let b = src + 2;
                if dst == a {
                    self.board[a + 1] = self.board[a - 2];
                    self.board[a - 2] = NO;
                } else if dst == b {
                    self.board[b - 1] = self.board[b + 1];
                    self.board[b + 1] = NO;
                }
            }
        }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn castle() {
        let mut bd = Chess::from_fen("r3k2rpppppppp8888PPPPPPPPR3K2R".to_string()).unwrap();
        assert!(bd.mv(60, 62).is_ok());
        dbg!(bd.to_fen());
        assert!(bd.mv(4, 6).is_ok());

        let mut bd = Chess::from_fen("r3k2rpppppppp8888PPPPPPPPR3K2R".to_string()).unwrap();
        assert!(bd.mv(60, 58).is_ok());
        dbg!(bd.to_fen());
        assert!(bd.mv(4, 2).is_ok());

        let mut bd = Chess::from_fen("r3k2rpppppppp8888PPPrPPPPR3K2R".to_string()).unwrap();
        dbg!(bd.is_dangerous(59));
        assert!(bd.mv(60, 58).is_err());
    }
}