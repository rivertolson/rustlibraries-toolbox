//! # toolbox
//! 
//! This crate is a group of useful code snippets
//! 
//! ## Modules
//! - strings
//! 
//! ## External Libraries
//! - `fastrand`

//! # strings
//!
//! A collection of functions that operate on strings.
//!
//! ## Features
//! - random_special
//!
//! ## Examples
//! ### random_special
//! Generates a random special character from the following:
//! ~`!@#$%^&*()_+-=[{]}\|;:'",<.>/?
//! 
//! ``` rust
//! use toolbox::strings::{random_special};
//! 
//! fn main() {
//!     let my_char: char = random_special();
//!     println!("'{}'", my_char);
//! }
//! ````
//!
//! ## Modules
//! - `fastrand`
pub mod strings{
    use fastrand;

    /// A character array of special characters
    pub const SPECIAL_CHARACTERS: [char;32] = [
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
    /// use toolbox::strings::{random_special};
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
}

pub mod numbers {
    pub fn max<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut max_num = &list[0];
        for item in list {
            if item > max_num {
                max_num = item;
            }
        }
        max_num
    }
}

#[cfg(test)]
mod random_special {
    use super::*;

    #[test]
    fn first_special_char() {
        let result = strings::SPECIAL_CHARACTERS[0];
        assert_eq!(result, '~');
    }

    #[test]
    fn last_special_char() {
        let result = strings::SPECIAL_CHARACTERS[31];
        assert_eq!(result, '?');
    }

    #[test]
    fn backslash_char() {
        let result = strings::SPECIAL_CHARACTERS[20];
        assert_eq!(result, '\\');
    }

    #[test]
    fn double_quote_char() {
        let result = strings::SPECIAL_CHARACTERS[24];
        assert_eq!(result, '\"');
    }

    #[test]
    fn quote_char() {
        let result = strings::SPECIAL_CHARACTERS[25];
        assert_eq!(result, '\'');
    }

    #[test]
    fn first_char_with_fastrand() {
        let ran_num: u8 = fastrand::u8(0..1);
        let result = strings::SPECIAL_CHARACTERS[ran_num as usize];
        assert_eq!(result, '~');
    }

    #[test]
    fn last_char_with_fastrand() {
        let ran_num: u8 = fastrand::u8(31..32);
        let result = strings::SPECIAL_CHARACTERS[ran_num as usize];
        assert_eq!(result, '?');
    }
}
