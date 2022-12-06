use crate::chess::{Ai, Chess, moves::Choice};

impl Ai {
    pub fn alphabetical(chess: Chess) -> Choice {
        let mut choices = chess.all_choices_note().into_iter().enumerate().collect::<Vec<(usize,String)>>();
        let unsorted = choices.clone();
        choices.sort_by(|(_,a),(_,b)| a.to_ascii_lowercase().cmp(&b.to_lowercase()));

        let (src,dst) = chess.from_std_notation(choices[0].1.clone()).unwrap();

        Choice::new(src, dst)
    }
}