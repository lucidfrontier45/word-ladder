pub fn get_first_char(s: &str) -> char {
    s.chars().next().unwrap()
}

pub fn get_last_char(s: &str) -> char {
    let last_char = s.chars().last().unwrap();
    if last_char == 'ー' {
        //
        let (last_idx, _) = s.char_indices().last().unwrap();
        let new_str = &s[..last_idx];
        return get_last_char(new_str);
    }
    match last_char {
        'ァ' => 'ア',
        'ィ' => 'イ',
        'ゥ' => 'ウ',
        'ェ' => 'エ',
        'ォ' => 'オ',
        'ッ' => 'ツ',
        'ャ' => 'ヤ',
        'ュ' => 'ユ',
        'ョ' => 'ヨ',
        _ => last_char,
    }
}

#[cfg(test)]
mod test {
    use super::{get_first_char, get_last_char};

    #[test]
    fn test_get_first_char() {
        assert_eq!(get_first_char("リンゴ"), 'リ');
    }

    #[test]
    fn test_get_last_char() {
        assert_eq!(get_last_char("リンゴ"), 'ゴ');
        assert_eq!(get_last_char("リンゴー"), 'ゴ');
        assert_eq!(get_last_char("リンゴゥ"), 'ウ');
    }
}
