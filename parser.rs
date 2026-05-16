use crate::errors::Error;

pub struct Parser;

impl Parser {
    /// Parses a slice of characters into an `f64` number, handling signs and decimal points.
    ///
    /// This function acts as the primary logic for numeric conversion. It validates the
    /// input structure, identifies the sign, and splits the number into integer and
    /// fractional parts without unnecessary memory allocations by using slices.
    ///
    /// # Algorithm
    /// 1. **Parser**: Checks for empty input, lone signs, or multiple decimal points.
    /// 2. **Sign Detection**: Determines if the number is negative based on the first character.
    /// 3. **Splitting**: Locates the decimal point and splits the slice into `first_part` and `second_part`.
    /// 4. **Conversion**: Calculates the final `f64` value using internal helper methods.
    ///
    /// # Errors
    /// Returns [`Error::TrailingSeparator`] if:
    /// * The input starts or ends with a decimal point (e.g., `.5` or `5.`).
    /// * The input is just a sign (`-`) or a single dot (`.`).
    /// * The structure is invalid (e.g., `-.5`).
    ///
    /// Returns [`Error::MultipleDots`] if:
    /// * More than one decimal point is present.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// # use your_crate::{Parser, Error};
    /// let input = vec!['-', '1', '0', '.', '5'];
    /// let result = Parser::parse_number(&input).unwrap();
    /// assert_eq!(result, -10.5);
    ///
    pub fn parse_number(number: &[char]) -> Result<f64, Error> {
        match number {
            ['-'] | ['.'] | ['.', '-'] | ['.', ..] | [.., '.'] => {
                return Err(Error::TrailingSeparator);
            }
            ['-', '.', rest @ ..] => return Err(Error::TrailingSeparator),
            _ if number.iter().filter(|char| char == &&'.').count() > 1 => {
                return Err(Error::MultipleDots);
            }
            _ => {}
        }

        let negative_number = number.first() == Some(&'-');

        let float_number = number.iter().any(|char| *char == '.');

        let start_index: usize = if negative_number { 1 } else { 0 };

        if float_number {
            let point = number[start_index..]
                .iter()
                .position(|char| char == &'.')
                .unwrap();
            dbg!(&point);

            let (first_part, second_part) = number[start_index..].split_at(point);

            dbg!((&first_part, &second_part));

            let final_number = Parser::to_float(first_part, &second_part[1..]);
            if negative_number {
                return Ok(-final_number);
            } else {
                return Ok(final_number);
            }
        } else {
            let final_number = Parser::to_number(&number[start_index..]);
            if negative_number {
                return Ok(-final_number);
            } else {
                return Ok(final_number);
            }
        }
    }

    /// Combines integer and fractional character slices into a single `f64`.
    ///
    /// This function calculates the final floating-point value by converting both
    /// parts to numbers and shifting the fractional part based on its length.
    ///
    /// # Arguments
    /// * `first_part` - Slice containing digits before the decimal point.
    /// * `second_part` - Slice containing digits after the decimal point.
    ///
    /// # Math Logic
    /// The result is computed as:
    /// $result = first\_number + (second\_number / 10^{second\_part.len()})$
    ///
    /// # Example
    /// ```
    /// # use your_crate::Parser;
    /// let whole = ['1', '0'];
    /// let frac = ['2', '5'];
    /// let result = Parser::to_float(&whole, &frac);
    /// assert_eq!(result, 10.25);
    ///
    fn to_float(first_part: &[char], second_part: &[char]) -> f64 {
        let power = second_part.len() as f64;

        let first_number = Parser::to_number(&first_part);

        let second_number = Parser::to_number(&second_part);

        let denominator = 10_f64.powf(power);

        first_number + (second_number / denominator)
    }
    /// Converts a slice of numeric characters into a 64-bit float.
    ///
    /// This helper function processes a slice of digits and builds the
    /// equivalent numeric value using a base-10 accumulator.
    ///
    /// # Panics
    ///
    /// This function will panic if:
    /// * Any character in the slice is not a decimal digit (0-9).
    /// * This includes decimal points `.` or signs `-`, which should be
    ///   handled by the caller before invoking this method.
    ///
    /// # Complexity
    ///
    /// * **Time complexity**: $O(n)$, where $n$ is the length of the slice.
    /// * **Memory complexity**: $O(1)$, as it operates directly on the slice.
    ///
    /// # Example
    ///
    /// ```
    /// # use your_crate::Parser;
    /// let digits = ['1', '2', '3'];
    /// let value = Parser::to_number(&digits);
    /// assert_eq!(value, 123.0);
    ///
    fn to_number(number: &[char]) -> f64 {
        number
            .iter()
            .fold(0.0, |acc, x| acc * 10.0 + x.to_digit(10).unwrap() as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_number {
        use super::*;

        #[test]
        fn test_parse_number_usual() {
            dbg!(Parser::parse_number(&['1', '0', '9']));
            assert!(Parser::parse_number(&['1', '0', '9']).is_ok());
        }
        #[test]
        fn test_parse_number_usual_neg() {
            dbg!(Parser::parse_number(&['-', '1', '0', '9']));
            assert!(Parser::parse_number(&['-', '1', '0', '9']).is_ok());
        }
        #[test]
        fn test_parse_number_usual_digit_neg() {
            dbg!(Parser::parse_number(&['-', '1']));
            assert!(Parser::parse_number(&['-', '1']).is_ok());
        }
        #[test]
        fn test_parse_number_usual_digit_neg_zeroes() {
            dbg!(Parser::parse_number(&['-', '0']));
            assert!(Parser::parse_number(&['-', '0']).is_ok());
        }

        #[test]
        fn test_parse_number_float() {
            dbg!(Parser::parse_number(&['1', '.', '0', '9']));
            assert!(Parser::parse_number(&['1', '.', '0', '9']).is_ok());
        }
        #[test]
        fn test_parse_number_float_neg() {
            dbg!(Parser::parse_number(&['-', '1', '.', '0', '9']));
            assert!(Parser::parse_number(&['-', '1', '.', '0', '9']).is_ok());
        }
        #[test]
        fn test_parse_number_second_point() {
            assert!(Parser::parse_number(&['-', '.', '.', '0', '9']).is_err());
        }
        #[test]
        fn test_parse_number_only_point_with_neg() {
            assert!(Parser::parse_number(&['-', '.']).is_err());
        }

        #[test]
        fn test_parse_number_only_point() {
            assert!(Parser::parse_number(&['.']).is_err());
        }
        #[test]
        fn test_parse_number_fitst_and_last_dots() {
            assert!(Parser::parse_number(&['.', '.']).is_err());
        }
        #[test]
        fn test_parse_number_middle_dots() {
            assert!(Parser::parse_number(&['1', '.', '2', '.', '3']).is_err());
        }
    }

    mod to_number {
        use super::*;
        #[test]
        fn test_to_number_10() {
            dbg!(Parser::to_number(&['1', '0']));
            assert_eq!(Parser::to_number(&['1', '0']), 10.0);
        }
        #[test]
        fn test_to_number_559() {
            dbg!(Parser::to_number(&['5', '5', '9']));
            assert_eq!(Parser::to_number(&['5', '5', '9']), 559.0);
        }

        #[test]
        fn negative_number() {
            dbg!(Parser::to_number(&['1']));
            assert_eq!(Parser::to_number(&['1']), 1.0);
        }
        #[test]
        fn big_negative_number() {
            dbg!(Parser::to_number(&['1', '2', '4', '9', '1']));
            assert_eq!(Parser::to_number(&['1', '2', '4', '9', '1']), 12491.0);
        }
    }

    mod to_float {
        use super::*;

        #[test]
        fn test_to_float() {
            dbg!(Parser::to_float(&['1', '2'], &['0', '7']));
            assert_eq!(Parser::to_float(&['1', '2'], &['0', '7']), 12.07)
        }
        #[test]
        fn zero_ahead() {
            dbg!(Parser::to_float(&['0'], &['5', '7']));
            assert_eq!(Parser::to_float(&['0'], &['5', '7']), 0.57)
        }
        #[test]
        fn only_zeroes() {
            dbg!(Parser::to_float(&['0', '0'], &['5', '7']));
            assert_eq!(Parser::to_float(&['0', '0'], &['5', '7']), 0.57)
        }
    }
}
