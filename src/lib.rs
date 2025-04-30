//! # toolbox
//! 
//! This crate is a group of useful code snippets
//! 
//! ## Functions
//! - random_special()
//! 
//! ## Examples
//! ### random_special
//! Generates a random special character from the following:
//! ~`!@#$%^&*()_+-=[{]}\|;:'",<.>/?
//! 
//! ``` rust
//! use toolbox::{random_special};
//! 
//! fn main() {
//!     let my_char: char = random_special();
//!     println!("'{}'", my_char);
//! }
//! ```
//! 
//! ## Modules
//! - `fastrand`

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

/// Generates a random special character from the following:
/// ~`!@#$%^&*()_+-=[{]}\|;:'",<.>/?
/// 
/// # Arguments
/// None
/// 
/// # Returns
/// char
/// 
/// # Examples
/// ``` rust
/// use toolbox::{random_special};
/// 
/// fn main() {
///     let my_char: char = random_special();
///     println!("'{}'", my_char);
/// }
/// ```
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
