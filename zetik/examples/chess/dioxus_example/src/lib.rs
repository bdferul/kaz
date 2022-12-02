use dioxus::prelude::*;
use zetik::chess::{ndx, Chess, fndx};
use zetik_tailwind::{twa,tailwind::classes::*};

#[derive(Default, Clone)]
struct ChessBoard {
    chess: Chess,
    selection: Option<usize>,
    log: Vec<String>,
}

pub fn app(cx: Scope<()>) -> Element {
    let chess_board = use_state(&cx, || ChessBoard::default());
    let hovering = use_state(&cx, || 0);

    cx.render(rsx!(
        div {
            style: twa!(text_amber_500, top_0, left_0, absolute, w_full, h_full, p_3),
            p {[format_args!("{}", chess_board.chess.to_fen())]}
            p {[format_args!("En passant: {:?}", chess_board.chess.en_passant)]}
            p {[format_args!("{hovering} : {:?}", fndx(**hovering))]}
            p {[format_args!("Selected: {:?}", chess_board.selection)]}
            table {
                style: twa![mx_auto, border, border_stone_800, text_yellow_100],
                (0..8).map(|y| rsx!(
                    tr {
                        (0..8).map(|x| rsx!(
                            td {
                                button {
                                    style: twa![w_12, h_12, "font-size: 1.875rem;", if chess_board.chess.knight_possible(**hovering).contains(&ndx(x,y)) {"background-color: rgb(252 165 165);"} else { "" }],
                                    onclick: move |_| chess_board.with_mut(|cb| cb.select(x,y)),
                                    onmouseover: move |_| hovering.modify(|_| ndx(x, y)),
                                    [format_args!("{}", Chess::to_symbol(chess_board.chess.board()[x+(8*y)],'-'))]
                                }
                            }
                        ))
                    }
                ))
            }
            div {
                class: "log",
                chess_board.log.iter().enumerate().map(|(i,msg)| rsx!(
                    "{i}: {msg}",
                    br {}
                ))
            }
        }
    ))
}

impl ChessBoard {
    pub fn select(&mut self, x: usize, y: usize) {
        let pos = ndx(x, y);
        if let Some(parent) = self.selection {
            if self.chess.mv(parent, pos).is_err() {
                self.log.push("invalid selection".to_string());
            }
            self.selection = None;
        } else {
            if self.chess.board()[pos].is_some() {
                self.selection = Some(pos)
            }
        }
    }
}
