use dioxus::{prelude::*, events::onclick};
use zetik::{chess::{ndx, Chess, fndx}, mdx};
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
    let input_fen = use_state(&cx, || String::new());
    let input_fen_error = use_state(&cx, || String::new());

    let cheat_highlighting = |x,y| {
        let mut r = if (x+y) & 1 == 1 {
            bg_fuchsia_300
        } else {
            bg_fuchsia_200
        };
        if chess_board.chess.possibilities(**hovering).0.contains(&mdx!(x,y)) {
        //if chess_board.chess.king_possible(**hovering).contains(&mdx!(x,y)) {
            r = bg_amber_500;
        }
        if let Some(a) = chess_board.selection {
            if chess_board.chess.possibilities(a).0.contains(&mdx!(x,y)) {
                r = bg_green_500;
            }
        }
        r
    };

    cx.render(rsx!(
        div {
            style: twa!(top_0, left_0, absolute, w_full, h_full, p_3, bg_stone_800, text_stone_300),
            p {[format_args!("{}", chess_board.chess.to_fen())]}
            "Input FEN: "
            input {
                style: twa![w_max],
                value: "{input_fen}",
                oninput: move |evt| input_fen.set(evt.value.clone()),
            }
            button {
                onclick: move |_| {
                    let from_fen = Chess::from_fen(input_fen.to_string());
                    if let Ok(new_board) = from_fen {
                        chess_board.with_mut(|cb| cb.chess = new_board);
                    } else {
                        input_fen_error.set(format!("{:?}", from_fen));
                    }
                },
                "submit"
            }
            "{input_fen_error}"
            p {[format_args!("En passant: {:?}", chess_board.chess.en_passant)]}
            p {[format_args!("Hover: {hovering} : {:?}", fndx(**hovering))]}
            p {[format_args!("Selected: {:?}", chess_board.selection)]}
            
            table {
                style: twa![mx_auto, border, border_stone_800, text_yellow_100],
                (0..8).map(|y| rsx!(
                    tr {
                        (0..8).map(|x| rsx!(
                            td {
                                button {
                                    style: twa![w_12, h_12, "font-size: 1.875rem;", cheat_highlighting(x,y)],
                                    onclick: move |_| chess_board.with_mut(|cb| cb.select(x,y)),
                                    onmouseover: move |_| hovering.modify(|_| ndx(x, y)),
                                    [format_args!("{}", Chess::to_symbol(chess_board.chess.board()[x+(8*y)],' '))]
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
        let src = mdx!(x,y);
        if let Some(a) = self.chess.board()[src] {
            if a.side == self.chess.turn {
                self.selection = Some(src);
                return;
            }
        }

        if let Some(a) = self.selection {
            if self.chess.mv(a, src).is_err() {
                self.log.push("invalid selection".to_string());
            }
        }
        /*
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
        */
    }
}
