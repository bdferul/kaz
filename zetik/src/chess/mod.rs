pub struct Chess {
    pub board: Vec<char>
}

impl Default for Chess {
    fn default() -> Self {
        Chess { 
            board: vec![' ';64] ,
        }
    }
}