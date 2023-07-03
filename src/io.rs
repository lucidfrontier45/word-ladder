use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// read token list from file
pub fn read_token_list(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut tokens = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        tokens.push(line);
    }
    tokens
}
