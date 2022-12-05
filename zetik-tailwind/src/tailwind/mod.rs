pub mod classes;

/// Takes a vector of tailwind classes (or just plain css) and joins them
pub fn tw_string(input: Vec<&str>) -> String {
    input
        .iter()
        .fold(String::new(), |a, s| [a, s.to_string()].join(" ")).trim().to_owned()
}

/// Takes the unformatted title of a class stripped straight from the tailwind docs and converts it to a format suitable for a variable declaration
pub fn fix_var_name(s: &str) -> String {
    s.replace("-", "_").replace(".", "_").replace("/", "__")
}

/// Accepts tailwind constants and returns them in string format for inline styling
#[macro_export]
macro_rules! tw {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            $crate::tailwind::tw_string(temp_vec)
        }
    };
}

/// returns the string output from the tw! macro as an argument
/// 
/// created for use with dioxus
#[macro_export]
macro_rules! twa {
    ( $( $x:expr ),* ) => {
        {
            format_args!("{}", $crate::tw![$($x),*])
        }
    };
}
pub(crate) use tw;
#[allow(unused)]
pub(crate) use twa;