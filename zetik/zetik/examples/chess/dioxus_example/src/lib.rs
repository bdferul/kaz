use dioxus::{prelude::*, events::{onmouseleave, onkeyup, KeyCode}};
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
    let hovering = use_state(&cx, || None);
    let input_fen = use_state(&cx, || String::from("r3k2rpppppppp8888PPPPPPPPR3K2R"));
    let input_fen_error = use_state(&cx, || String::new());
    let input_std_note = use_state(&cx, || String::new());

    let cheat_highlighting = |x: usize,y: usize| {
        let mut r = if (x+y) & 1 == 1 {
            bg_fuchsia_400
        } else {
            bg_fuchsia_200
        };
        if let Some(h) = **hovering {
            if chess_board.chess.choices(h).contains(&mdx!(x,y)) {
            //if chess_board.chess.king_possible(**hovering).contains(&mdx!(x,y)) {
                r = bg_amber_500;
            }
        }
        
        if let Some(a) = chess_board.selection {
            if chess_board.chess.choices(a).contains(&mdx!(x,y)) {
                r = bg_green_500;
            }
        }
        r
    };

    cx.render(rsx!(
        div {
            style: twa!(top_0, left_0, absolute, w_full, h_full, p_3, bg_stone_800, text_stone_300, overflow_scroll),
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
            p {[format_args!("Hover: {hovering:?} : {:?}", if let Some(h) = **hovering {Some((fndx(h), Chess::fen_pos(h).unwrap()))} else {None})]}
            p {[format_args!("Selected: {:?}", chess_board.selection)]}
            p {[format_args!("Check: {:?}", chess_board.chess.check)]}
            p {[format_args!("Checkmate: {}", chess_board.chess.checkmate)]}
            p {[format_args!("Stalemate: {}", chess_board.chess.stalemate)]}
            div {
                style: twa![mx_auto, flex, h_min],
                table {
                    style: twa![border, border_stone_800, text_yellow_100, h_min],
                    (0..8).map(|y| rsx!(
                        tr {
                            (0..8).map(|x| rsx!(
                                td {
                                    button {
                                        style: twa![w_12, h_12, "font-size: 1.875rem;", cheat_highlighting(x,y)],
                                        onclick: move |_| chess_board.with_mut(|cb| cb.select(x,y)),
                                        onmouseover: move |_| hovering.modify(|_| Some(ndx(x, y))),
                                        onmouseout: move |_| hovering.modify(|_| None),
                                        [format_args!("{}", Chess::to_symbol(chess_board.chess.board()[x+(8*y)],' '))]
                                    }
                                }
                            ))
                        }
                    ))
                }
                    div {
                    style: twa![overflow_y_scroll, h_auto],
                    "Input move"
                    input {
                        value: "{input_std_note}",
                        oninput: move |evt| input_std_note.set(evt.value.clone()),
                        onkeyup: move |evt| {
                            if evt.key_code == KeyCode::Enter {
                                chess_board.with_mut(|cb| {cb.chess.mv_str(input_std_note.clone().to_string());});
                                input_std_note.set(String::new());
                            }
                        },
                    }
                    chess_board.chess.move_log.iter().enumerate().map(|(i,msg)| rsx!(
                        "{i}. {msg}",
                        br {}
                    ))
                }
                
            }
            div {
                class: "log",
                {
                    let mut acn = chess_board.chess.all_choices_note();
                    for a in acn.iter_mut() {
                        *a = a.to_ascii_uppercase();
                    }
                    acn.sort();
                    acn.into_iter().enumerate().map(|(i,msg)| rsx!(
                    "{i}: {msg}",
                    br {}
                    ))
                }
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

            self.selection = None
        }
    }
}
