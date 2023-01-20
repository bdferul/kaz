use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, Texture},
};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn main() {
    App::new().insert_resource(ClearColor(Color::BLACK)).run();
}
