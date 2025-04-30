use fastrand;

const SPECIAL_CHARACTERS: [char;32] = [
    '~',
    '`',
    '!',
    '@',
    '#',
    '$',
    '%',
    '^',
    '&',
    '*',
    '(',
    ')',
    '-',
    '_',
    '=',
    '+',
    '[',
    '{',
    ']',
    '}',
    '\\',
    '|',
    ';',
    ':',
    '\"',
    '\'',
    ',',
    '<',
    '.',
    '>',
    '/',
    '?',
];

pub fn random_special() -> char {
    let ran_num: u8 = fastrand::u8(0..32);
    SPECIAL_CHARACTERS[ran_num as usize]
}

#[cfg(test)]
mod random_special {
    use super::*;

    #[test]
    fn first_special_char() {
        let result = SPECIAL_CHARACTERS[0];
        assert_eq!(result, '~');
    }

    #[test]
    fn last_special_char() {
        let result = SPECIAL_CHARACTERS[31];
        assert_eq!(result, '?');
    }

    #[test]
    fn backslash_char() {
        let result = SPECIAL_CHARACTERS[20];
        assert_eq!(result, '\\');
    }

    #[test]
    fn double_quote_char() {
        let result = SPECIAL_CHARACTERS[24];
        assert_eq!(result, '\"');
    }

    #[test]
    fn quote_char() {
        let result = SPECIAL_CHARACTERS[25];
        assert_eq!(result, '\'');
    }

    #[test]
    fn first_char_with_fastrand() {
        let ran_num: u8 = fastrand::u8(0..1);
        let result = SPECIAL_CHARACTERS[ran_num as usize];
        assert_eq!(result, '~');
    }

    #[test]
    fn last_char_with_fastrand() {
        let ran_num: u8 = fastrand::u8(31..32);
        let result = SPECIAL_CHARACTERS[ran_num as usize];
        assert_eq!(result, '?');
    }
}
