use seed::{prelude::*, *};
use zetik::chess::{Chess};
struct Model {
    counter: i32,
    chess: Chess,
    selected: Option<usize>,
    mv_res: String,
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        counter: 0,
        chess: Chess::default(),
        selected: None,
        mv_res: String::from("_"),
    }
}

enum Msg {
    Increment,
    Select(usize),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
        Msg::Select(xy) => {
            let Some(sxy) = model.selected else {
                model.selected = Some(xy);
                return
            };

            if let Some(p) = model.chess.board()[xy] {
                if model.chess.turn == p.side {
                    model.selected = Some(xy);
                    return
                }
            }

            let res = model.chess.mv(sxy,xy);
            model.mv_res = format!("{:?}", res);
            if res.is_ok() {
                model.selected = None;
            }
        } 
    }
}

fn ndx(x: usize, y: usize) -> usize {
    x + (8*y)
}

fn view(model: &Model) -> Node<Msg> {
    let checkmate = model.chess.checkmate;
    div![
        //tw
        C!["bg-stone-900 text-stone-400 absolute top-0 bottom-0 right-0 left-0 font-mono"],
        "This is a counter: ",
        button![model.counter, ev(Ev::Click, |_| Msg::Increment),],
        p![format!("{:?}", model.selected)],
        p![model.mv_res.clone()],
        p![if checkmate {"Checkmate!"} else {""} ],
        table![
            tr![
                td![],
                ('a'..='h').map(|x| td![
                    //tw
                    C!["border-b border-stone-700"],
                    x.to_string(),
                ]),
            ],
            (0..8).map(|y| tr![
                td![
                    //tw
                    C!["border-r border-stone-700"],
                    y+1,
                ],
                (0..8).map(|x| {
                    let xy = ndx(x,y);
                    td![
                        //tw
                        C![format!("p-1 w-10 h-10 {}", if (x+y) % 2 == 0 {"bg-stone-800"} else {""})],
                        format!("{}", Chess::to_symbol(model.chess.board()[xy], ' ')),
                        ev(Ev::Click, move |_| Msg::Select(xy))
                    ]
                })
            ])
        ],
        model.chess.move_log.iter().map(|m| p![m]),
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}