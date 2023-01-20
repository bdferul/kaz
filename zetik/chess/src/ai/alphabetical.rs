use crate::{ai::Ai, Chess};

impl Ai {
    pub fn alphabetical(chess: Chess) -> (usize, usize) {
        let mut choices = chess
            .all_choices_note()
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, String)>>();
        choices.sort_by(|(_, a), (_, b)| a.to_ascii_lowercase().cmp(&b.to_lowercase()));

        let (src, dst) = chess.from_std_notation(&choices[0].1).unwrap();

        (src, dst)
    }
}
