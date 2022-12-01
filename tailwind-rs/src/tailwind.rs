type T = &'static str;

pub const mx_auto: T = "margin-left: auto; margin-right: auto;";
pub const whitespace_nowrap: T = "white-space: nowrap;";

pub fn tw(input: Vec<&str>) -> String {
    input
        .iter()
        .fold(String::new(), |a, s| [a, s.to_string()].join(" "))
}
