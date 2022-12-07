use crate::chess::Chess;

impl Chess {
    pub fn notate(&self, src: usize, dst: usize) -> Option<String> {
        let Some(src_p) = self.board[src] else {
            return None;
        };

        None
    }
}