pub fn get_first_char(s: &str) -> char {
    s.chars().next().unwrap()
}

pub fn get_last_char(s: &str) -> char {
    s.chars().last().unwrap()
}
