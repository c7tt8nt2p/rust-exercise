#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    let cc = cc_number.trim().to_owned();
    let Ok(cc) = validate_length(cc) else { return false; };
    let Ok(cc) = validate_numeric(cc) else { return false; };
    let Ok(cc) = double_every_second_digit(cc) else { return false; };
    true
}

fn validate_length(s: String) -> Result<String, ()> {
    if s.len() < 2 {
        return Err(());
    }
    Ok(s)
}

fn validate_numeric(s: String) -> Result<String, ()> {
    let is_numeric = s
        .chars()
        .filter(|e| !e.is_whitespace())
        .all(|e| e.is_ascii_digit());
    if is_numeric {
        return Ok(s);
    }
    Err(())
}

fn double_every_second_digit(s: String) -> Result<String, ()> {
    let mut v: Vec<char> = s.chars().rev().filter(|e| !e.is_whitespace()).collect();

    let double_digit_fnc = |i, e: &mut char| {
        if i % 2 == 0 {
            return e.to_digit(10).unwrap() as u8;
        }
        let mut num = e.to_digit(10).unwrap() as u8;
        num *= 2;
        if num > 9 {
            num = num - 10 + 1;
        }
        num
    };
    let sum: u8 = v
        .iter_mut()
        .enumerate()
        .map(|(i, e)| double_digit_fnc(i, e))
        .sum();

    if sum.to_string().ends_with('0') {
        return Ok(s);
    }
    Err(())
}

#[cfg(test)]
mod tests {
    use crate::luhn;

    #[test]
    fn test_non_digit_cc_number() {
        assert!(!luhn("foo"));
    }

    #[test]
    fn test_empty_cc_number() {
        assert!(!luhn(""));
        assert!(!luhn(" "));
        assert!(!luhn("  "));
        assert!(!luhn("    "));
    }

    #[test]
    fn test_single_digit_cc_number() {
        assert!(!luhn("0"));
    }

    #[test]
    fn test_two_digit_cc_number() {
        assert!(luhn(" 0 0 "));
    }

    #[test]
    fn test_valid_cc_number() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
    }
}

#[allow(dead_code)]
fn main() {}
