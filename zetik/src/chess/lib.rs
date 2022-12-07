pub fn ndx(x: usize, y: usize) -> usize {
    x + (y * 8)
}

pub fn fndx(p: usize) -> (usize, usize) {
    (p % 8, p / 8)
}

/// use like `mdx!(x: int, y: int)`
///
/// returns usize
///
/// this is beautiful
#[macro_export]
macro_rules! mdx {
    ($x:expr, $y:expr) => {{
        if $x.to_owned() > 7 {
            panic!("x is too large: {}", $x);
        }
        if $y.to_owned() > 7 {
            panic!("y is too large: {}", $y);
        }
        ($x.to_owned() as usize + ($y.to_owned() as usize * 8))
    }};
    ($x:expr, $y:expr, $t:ty) => {{
        if $x > 7 {
            panic!("x is too large: {}", $x);
        }
        if $y > 7 {
            panic!("y is too large: {}", $y);
        }
        ($x as $t + ($y as $t * 8))
    }};
}
#[allow(unused)]
pub(crate) use mdx;

#[macro_export]
macro_rules! fmdx {
    ($x:expr) => {
        ($x as usize % 8, $x as usize / 8)
    };
    ($x:expr, $t:ty) => {
        ($x as $t % (8 as $t), $x as $t / (8 as $t))
    };
}
#[allow(unused)]
pub(crate) use fmdx;

/// low <= x < high
#[macro_export]
macro_rules! in_range {
    ($low:expr, $x:expr, $high:expr) => {
        $low <= $x && $x < $high
    };
}
#[allow(unused)]
pub(crate) use in_range;

/// Acts like the standard println macro, but it prints leading with the line number
#[macro_export]
macro_rules! pln {
    () => {
        $crate::print!("\n")
    };
    ($fmt:expr) => {{
        print!("{}: ", line!());
        print!($fmt);
        println!("\t{}:{}:{}", file!(), line!(), column!());
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        print!("{}: ", line!());
        print!($fmt, $($arg)*);
        println!("\t{}:{}:{}", file!(), line!(), column!());
    }};
}
#[allow(unused_imports)]
pub(crate) use pln;
