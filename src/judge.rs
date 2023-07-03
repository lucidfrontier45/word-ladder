use std::collections::{HashMap, HashSet};

fn get_first_char(s: &str) -> char {
    s.chars().next().unwrap()
}

fn get_last_char(s: &str) -> char {
    s.chars().last().unwrap()
}

#[derive(Debug, PartialEq, Eq)]
pub enum JudgeResult {
    // game is not over
    Continue,
    // current player is the winner
    Win,
    // current player is the loser
    Lose,
}

pub struct Judge {
    remained_token_map: HashMap<char, HashSet<String>>,
}

impl Judge {
    pub fn new(tokens: impl IntoIterator<Item = String>) -> Self {
        let mut remained_token_map: HashMap<char, HashSet<String>> = HashMap::new();
        for token in tokens {
            let first_char = get_first_char(&token);
            let token_set = remained_token_map
                .entry(first_char)
                .or_insert_with(HashSet::new);
            token_set.insert(token);
        }
        Self { remained_token_map }
    }

    fn remove_token(&mut self, token: &str) {
        let first_char = get_first_char(token);
        let token_set = self.remained_token_map.get_mut(&first_char).unwrap();
        token_set.remove(token);
    }

    pub fn make_judgement(&mut self, token: &str) -> JudgeResult {
        // check if given token exists in the remained map
        let first_char = get_first_char(token);
        if !self.remained_token_map.contains_key(&first_char) {
            return JudgeResult::Lose;
        }

        // update remained map
        self.remove_token(token);

        // check if there is any other token that starts with the last char of given token
        let last_char = get_last_char(token);
        let remained = self.remained_token_map.get(&last_char);
        if remained.map(|s| s.is_empty()).unwrap_or(true) {
            JudgeResult::Win
        } else {
            JudgeResult::Continue
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    #[test]
    fn test() {
        let words = HashSet::from([
            "リンゴ".to_string(),
            "ゴリラ".to_string(),
            "ラッパ".to_string(),
        ]);

        let mut judge = super::Judge::new(words);

        assert_eq!(judge.make_judgement("パンダ"), super::JudgeResult::Lose);
        assert_eq!(judge.make_judgement("リンゴ"), super::JudgeResult::Continue);
        assert_eq!(judge.make_judgement("ゴリラ"), super::JudgeResult::Continue);
        assert_eq!(judge.make_judgement("ラッパ"), super::JudgeResult::Win);
    }
}
