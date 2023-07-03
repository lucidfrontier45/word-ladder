pub trait Agent {
    fn next_token(&mut self, token: &str) -> Option<String>;
}
