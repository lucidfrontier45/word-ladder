use std::collections::{HashMap, HashSet};

use crate::judge::Rule;
use crate::utils::{get_first_char, get_last_char};

use super::Agent;

pub struct SimpleAgent {
    remained_token_map: HashMap<char, HashSet<String>>,
}

impl SimpleAgent {
    pub fn new(rule: Rule) -> Self {
        let mut remained_token_map: HashMap<char, HashSet<String>> = HashMap::new();
        for token in rule.tokens {
            let last_char = get_last_char(&token);
            if rule.invalid_last_chars.contains(&last_char) {
                continue;
            }
            let first_char = get_first_char(&token);
            let token_set = remained_token_map
                .entry(first_char)
                .or_insert_with(HashSet::new);
            token_set.insert(token);
        }
        Self { remained_token_map }
    }
}

impl Agent for SimpleAgent {
    fn next_token(&mut self, token: &str) -> Option<String> {
        // remove given token from remained map
        let first_char = get_first_char(token);
        let token_set = self.remained_token_map.get_mut(&first_char).unwrap();
        token_set.remove(token);

        // get next token
        let last_char = get_last_char(token);
        let token_set = self.remained_token_map.get(&last_char);
        let next_token = token_set
            .and_then(|s| s.iter().next())
            .map(|s| s.to_string())?;

        let first_char = get_first_char(next_token.as_str());
        let token_set = self.remained_token_map.get_mut(&first_char).unwrap();
        token_set.remove(next_token.as_str());

        Some(next_token)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::agent::Agent;
    use crate::judge::Rule;

    #[test]
    fn test() {
        let words = HashSet::from([
            "リンゴ".to_string(),
            "ゴリラ".to_string(),
            "ラッパ".to_string(),
        ]);
        let rule = Rule::new(words);

        let mut agent = super::SimpleAgent::new(rule);
        let next_token = agent.next_token("リンゴ").unwrap();
        assert_eq!(next_token, "ゴリラ");
        let next_token = agent.next_token("ラッパ");
        assert!(next_token.is_none());
    }
}
