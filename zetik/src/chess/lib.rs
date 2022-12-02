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
        if $x > 7 {
            panic!("x is too large: {}", $x);
        }
        if $y > 7 {
            panic!("y is too large: {}", $y);
        }
        ($x as usize + ($y as usize * 8))
    }};
}
pub(crate) use mdx;

#[macro_export]
macro_rules! fmdx {
    ($x:expr, $t:ty) => {
        ($x as $t % 8, $x as $t / 8)
    };
}
pub(crate) use fmdx;
