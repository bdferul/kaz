#[cfg(test)]
mod tests {
    #[test]
    fn addition() {
        assert_eq!(lisp!("(3+4)"), 7);
    }
}
