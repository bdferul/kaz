use seed::{prelude::*, *};
use zetik::chess::Chess;
struct Model {
    counter: i32,
    chess: Chess,
    selected: Option<usize>,
    hover: Option<usize>,
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        counter: 0,
        chess: Chess::default(),
        selected: None,
        hover: None,
    }
}

enum Msg {
    Increment,
    Select(usize),
    MouseEnter(usize),
    MouseLeave(usize),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
        Msg::Select(xy) => {
            let Some(sxy) = model.selected else {
                if selectable(xy, &model) {
                    model.selected = Some(xy);
                }
                return
            };

            if xy == sxy {
                model.selected = None;
                return
            }

            if let Some(p) = model.chess.board()[xy] {
                if model.chess.turn == p.side {
                    model.selected = Some(xy);
                    return;
                }
            }

            if model.chess.mv(sxy, xy).is_ok() {
                model.selected = None;
            }
        },
        Msg::MouseEnter(xy) => {
            model.hover = Some(xy);
        },
        Msg::MouseLeave(xy) => {
            if let Some(h) = model.hover {
                if xy == h {
                    model.hover = None
                }
            }
        }
    }
    
}

fn square_color(x: usize, y: usize, model: &Model) -> &'static str {
    let xy = ndx(x, y);
    if let Some(selected) = model.selected {
        let choices = model.chess.choices(selected);
        if xy == selected {//tw
            return "bg-green-600 text-green-200"
        }
        if choices.contains(&xy) {
            return "tw bg-yellow-400 text-yellow-700 border border-yellow-600"
        }
    }
    if (x+y) & 1 != 1 {
        "tw bg-stone-700"
    } else {
        ""
    }
}

fn selectable(index: usize, model: &Model) -> bool {
    let Some(p) = model.chess.board()[index] else {
        return false
    };

    if p.side == model.chess.turn {
        true
    } else {
        false
    }
} 

fn ndx(x: usize, y: usize) -> usize {
    x + (8 * y)
}

fn view(model: &Model) -> Node<Msg> {
    let checkmate = model.chess.checkmate;
    div![
        C!["bg-stone-900 text-stone-400 absolute top-0 bottom-0 right-0 left-0 font-mono"],
        div![
            C!["mx-auto max-w-7xl"],
            "This is a counter: ",
            button![model.counter, ev(Ev::Click, |_| Msg::Increment),],
            p![format!("hover: {:?}", model.hover)],
            p![if checkmate { "Checkmate!" } else { "" }],
            table![
                C!["text-center align-middle select-none"],
                tr![
                    td![],
                    ('a'..='h').map(|x| td![
                        C!["border-b border-stone-700"],
                        x.to_string(),
                    ]),
                ],
                (0..8).map(|y| tr![
                    td![//tw
                        C!["border-r border-stone-700 pr-2"],
                        y + 1,
                    ],
                    (0..8).map(|x| {
                        let xy = ndx(x, y);
                        td![C![format!(//tw
                                "tw p-1 w-16 h-16 text-4xl {} {}",
                                square_color(x, y, model),
                                if selectable(xy, &model) {//tw
                                    "cursor-pointer"
                                } else {""}
                            )],
                            format!("{}", Chess::to_symbol(model.chess.board()[xy], ' ')),
                            ev(Ev::Click, move |_| Msg::Select(xy)),
                            ev(Ev::MouseEnter, move |_| Msg::MouseEnter(xy)),
                            ev(Ev::MouseLeave, move |_| Msg::MouseLeave(xy)),
                        ]
                    }),
                    td![
                        //tw
                        C!["tw border-l border-stone-700 pl-2"],
                        y + 1,
                    ],
                ])
            ],
            p![format!("FEN: {}", model.chess.to_fen())],
            model.chess.move_log.iter().map(|m| p![m]),
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
