use std::fmt::Display;
use termion::style;

pub trait StyleExt {
    fn bold(&self) -> String;
    fn italic(&self) -> String;
}

fn styled<S: Display, T: Display>(s: S, b: T) -> String {
    format!("{}{}{}", b, s, style::Reset)
}

impl<T: Display> StyleExt for T {
    fn bold(&self) -> String {
        styled(self, style::Bold)
    }

    fn italic(&self) -> String {
        styled(self, style::Italic)
    }
}
