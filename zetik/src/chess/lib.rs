pub fn ndx(x: usize, y: usize) -> usize {
    x + (y * 8)
}

pub fn fndx(p: usize) -> (usize, usize) {
    (p % 8, p / 8)
}
