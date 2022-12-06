use std::net::SocketAddr;
use axum::{response::{Json}, Router, routing::get};
use the_library::UserData;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_data));

    // run it with hyper
    let addr = SocketAddr::from(([0,0,0,0], 3000));
    //tracing::debug!("listening on {}", addr);
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_data() -> Json<UserData> {
    let jahy = UserData::new(1, "The Don", "Denmark");

    dbg!(&jahy);

    Json(jahy)
}