#[macro_export]
macro_rules! pln {
    () => {
        println!("{}:{}:{}", file!(), line!(), column!())
    };
    ($fmt:expr) => {{
        print!("{}. ", line!());
        print!($fmt);
        println!("\t{}:{}:{}", file!(), line!(), column!());
    }};
    ($fmt:expr, $($args:tt)*) => {{
        print!("{}. ", line!());
        print!($fmt, $($args)*);
        println!("\t{}:{}:{}", file!(), line!(), column!());
    }};
}
