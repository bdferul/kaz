/// Accepts a set of x,y coordinates(and an optional type T) and returns a single usize(or T) of x*8y
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

/// Accepts a single usize(and an optional type T) and returns  a tuple (x%8,x/8) as usize(or T)
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
