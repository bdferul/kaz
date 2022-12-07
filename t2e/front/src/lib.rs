use seed::{prelude::*, *};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        counter: 0,
    }
}

struct Model {
    counter: i32,
}

enum Msg {
    Increment,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
    }
}

fn ndx(x: usize, y: usize) -> usize {
    x + (8*y)
}

fn view(model: &Model) -> Node<Msg> {
    div![
        //tw
        C!["bg-gray-900 text-gray-400 absolute top-0 bottom-0 right-0 left-0"],
        "This is a counter: ",
        button![model.counter, ev(Ev::Click, |_| Msg::Increment),],
        table![
            tr![
                td![],
                (0..8).map(|x| td![
                    //tw
                    C!["border-b border-gray-700"],
                    x+1,
                ]),
            ],
            (0..8).map(|y| tr![
                td![
                    //tw
                    C!["border-r border-gray-700"],
                    y+1,
                ],
                (0..8).map(|x| td![
                    //tw
                    C!["p-1"],
                    format!("{}", ndx(x,y))
                ])
            ])
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}