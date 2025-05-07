//! # toolbox
//! 
//! This crate is a group of useful code snippets
//! 
//! ## Modules
//! - strings
//! 
//! ## External Libraries
//! - `fastrand`

/// # strings
///
/// A collection of functions that operate on strings.
///
/// ## Features
/// - random_special
///
/// ## Examples
/// ### random_special
/// Generates a random special character from the following:
/// ~`!@#$%^&*()_+-=[{]}\|;:'",<.>/?
/// 
/// ``` rust
/// use toolbox::strings::{random_special};
/// 
/// fn main() {
///     let my_char: char = random_special();
///     println!("'{}'", my_char);
/// }
/// ````
///
/// ## Modules
/// - `fastrand`
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

/// # numbers
/// 
/// Gets the largest number from a list
/// 
/// ## Features
/// - max
/// - min
/// 
/// ## Examples
/// ### max
/// Gets the largest number from a list
/// ``` rust
/// use toolbox::numbers::{max};
/// 
/// fn main() {
///     let number_list = vec![10, 3, 18, 5, 2];
///     let highest_number = max(&number_list);
///     assert_eq!(highest_number, &18);
/// }
/// ```
/// 
/// ### min
/// Gets the smallest number from a list
/// ``` rust
/// use toolbox::numbers::{min};
/// 
/// fn main() {
///     let number_list = vec![10, 3, 18, 5, 2];
///     let lowest_number = max(&number_list);
///     assert_eq!(highest_number, &2);
/// }
/// ```
pub mod numbers {
    /// Gets the largest number from a list
    /// 
    /// # Arguments
    /// list: &[T]
    /// 
    /// # Returns
    /// &T
    /// 
    /// # Examples
    /// ``` rust
    /// use toolbox::numbers::{max};
    /// 
    /// fn main() {
    ///     let number_list = vec![10, 3, 18, 5, 2];
    ///     let highest_number = max(&number_list);
    ///     assert_eq!(highest_number, &18);
    /// }
    /// ```
    pub fn max<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut max_num = &list[0];
        for item in list {
            if item > max_num {
                max_num = item;
            }
        }
        max_num
    }

    /// Gets the smallest number from a list
    /// 
    /// # Arguments
    /// list: &[T]
    /// 
    /// # Returns
    /// &T
    /// 
    /// # Examples
    /// ``` rust
    /// use toolbox::numbers::{min};
    /// 
    /// fn main() {
    ///     let number_list = vec![10, 3, 18, 5, 2];
    ///     let lowest_number = max(&number_list);
    ///     assert_eq!(highest_number, &2);
    /// }
    /// ```
    pub fn min<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut min_num = &list[0];
        for item in list {
            if item < min_num {
                min_num = item;
            }
        }
        min_num
    }

    /// Gets the average of a list of numbers
    /// 
    /// # Arguments
    /// list: &[T]
    /// 
    /// # Returns
    /// f64
    /// 
    /// # Examples
    /// ``` rust
    /// use toolbox::numbers::{avg};
    /// 
    /// fn main() {
    ///     let list: Vec<u8> = vec![1, 2, 3, 4, 5];
    ///     let list_avg = avg(&list);
    ///     assert_eq!(list_avg, 3.0);
    /// }
    /// ```
    pub fn avg<T: std::cmp::PartialOrd + Clone>(list: &[T]) -> f64 where f64: From<T> {
        let count = list.len() as f64;
        let mut total: f64 = 0.0;
        for item in list {
            total += f64::from(item.clone());
        }
        total / count
    }

    /// Gets the median value of a list
    /// 
    /// # Arguments
    /// list &mut [T]
    /// 
    /// # Returns
    /// f64
    /// 
    /// # Examples
    /// ``` rust
    /// use toolbox::numbers::{median};
    /// 
    /// fn main() {
    ///     let mut list: Vec<i16> = vec![8, 4, 2, 1, 3];
    ///     let list_median = median(&mut list)
    ///     assert_eq!(list_median, 3.0);
    /// }
    /// ```
    pub fn median<T: std::cmp::PartialOrd + Clone + std::cmp::Ord>(list: &mut [T]) -> f64 where f64: From<T> {
        list.sort();
        let count = list.len();
        if count % 2 == 0 {
            return f64::from(list[count/2].clone()) + f64::from(list[(count/2)-1].clone());
        }
        else {
            return f64::from(list[count/2].clone());
        }
    }

    /// Gets the mode value of a list of numbers
    /// @TODO
    /// # Arguments
    pub fn mode<T: std::cmp::PartialOrd + Clone + std::cmp::Ord>(list: &mut [T]) -> f64 where f64: From<T> {
        list.sort();
        // let value_list = Vec::new();
        // for item in list {
        //     for value in value_list {
        //         if item = value.0
        //     }
        // }
        0.0
    }
}

#[cfg(test)]
mod max {
    use crate::numbers::max;

    #[test]
    fn highest_number_in_list_is_twenty() {
        let my_list = vec![1, 3, 20, 15];
        let max_num = max(&my_list);
        assert_eq!(max_num, &20);
    }

    #[test]
    fn highest_number_in_list_is_negative_twenty() {
        let my_list = vec![-56, -48, -34, -20, -67];
        let max_num = max(&my_list);
        assert_eq!(max_num, &-20);
    }

    #[test]
    fn highest_number_in_list_is_ten_point_2() {
        let my_list = vec![-2.4, 10.2, 8.0, -4.1];
        let max_num = max(&my_list);
        assert_eq!(max_num, &10.2);
    }
}

mod min {
    use crate::numbers::min;

    #[test]
    fn lowest_number_in_list_is_one() {
        let my_list = vec![1, 3, 20, 15];
        let min_num = min(&my_list);
        assert_eq!(min_num, &1);
    }

    #[test]
    fn lowest_number_in_list_is_negative_sixtyseven() {
        let my_list = vec![-56, -48, -34, -20, -67];
        let min_num = min(&my_list);
        assert_eq!(min_num, &-67);
    }

    #[test]
    fn lowest_number_in_list_is_negative_four_point_one() {
        let my_list = vec![-2.4, 10.2, 8.0, -4.1];
        let min_num = min(&my_list);
        assert_eq!(min_num, &-4.1);
    }
}

mod avg {
    use crate::numbers::avg;

    #[test]
    fn average_of_list_is_three() {
        let my_list = vec![1, 2, 3, 4, 5];
        let list_average = avg(&my_list);
        assert_eq!(list_average, 3.0);
    }
}

mod random_special {
    use crate::strings;

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
