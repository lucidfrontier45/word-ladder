use std::collections::{HashMap, HashSet};

use crate::utils::{get_first_char, get_last_char};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rule {
    pub tokens: HashSet<String>,
    pub invalid_last_chars: HashSet<char>,
}

impl Rule {
    pub fn new(tokens: impl IntoIterator<Item = String>) -> Self {
        Self {
            tokens: tokens.into_iter().collect(),
            invalid_last_chars: HashSet::new(),
        }
    }

    pub fn with_invalid_last_chars(
        tokens: impl IntoIterator<Item = String>,
        invalid_last_chars: impl IntoIterator<Item = char>,
    ) -> Self {
        Self {
            tokens: tokens.into_iter().collect(),
            invalid_last_chars: invalid_last_chars.into_iter().collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum JudgeResult {
    // game is not over
    Continue,
    // current player is the loser
    Lose,
}

pub struct Judge {
    remained_token_map: HashMap<char, HashSet<String>>,
    invalid_last_chars: HashSet<char>,
}

impl Judge {
    pub fn new(rule: Rule) -> Self {
        let Rule {
            tokens,
            invalid_last_chars,
        } = rule;
        let mut remained_token_map: HashMap<char, HashSet<String>> = HashMap::new();
        for token in tokens {
            let first_char = get_first_char(&token);
            let token_set = remained_token_map
                .entry(first_char)
                .or_insert_with(HashSet::new);
            token_set.insert(token);
        }
        Self {
            remained_token_map,
            invalid_last_chars,
        }
    }

    fn remove_token(&mut self, token: &str) {
        let first_char = get_first_char(token);
        let token_set = self.remained_token_map.get_mut(&first_char).unwrap();
        token_set.remove(token);
    }

    pub fn make_judgement(&mut self, token: &str) -> JudgeResult {
        // check if given token is empty
        if token.is_empty() {
            return JudgeResult::Lose;
        }

        // check if given token ends with invalid last char
        let last_char = get_last_char(token);
        if self.invalid_last_chars.contains(&last_char) {
            return JudgeResult::Lose;
        }

        // check if given token exists in the remained map
        let first_char = get_first_char(token);
        if self
            .remained_token_map
            .get(&first_char)
            .map(|tokens| !tokens.contains(token))
            .unwrap_or(false)
        {
            return JudgeResult::Lose;
        }

        // update remained map
        self.remove_token(token);
        JudgeResult::Continue
    }
}

#[cfg(test)]
mod test {
    use crate::judge::{Judge, Rule};

    #[test]
    fn test() {
        let words = [
            "リンゴ".to_string(),
            "ゴリラ".to_string(),
            "ラッパ".to_string(),
            "パン".to_string(),
        ];

        let rule = Rule::with_invalid_last_chars(words, ['ン']);

        let mut judge = Judge::new(rule);

        assert_eq!(judge.make_judgement("パン"), super::JudgeResult::Lose);
        assert_eq!(judge.make_judgement("パンダ"), super::JudgeResult::Lose);
        assert_eq!(judge.make_judgement("リンゴ"), super::JudgeResult::Continue);
        assert_eq!(judge.make_judgement("ゴリラ"), super::JudgeResult::Continue);
    }
}
