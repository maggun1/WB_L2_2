use std::io::{Error, ErrorKind};

fn unpack_string(str: &String) -> Result<String, Error> {
    let mut result = String::new();
    let mut symbol = String::new();
    let mut escaped = false;

    for c in str.chars() {
        if escaped {
            if !symbol.is_empty() {
                result.push_str(symbol.as_str());
                symbol.clear();
            }
            symbol.push(c);
            escaped = false;
            continue;
        }

        match c {
            '\\' => {
                escaped = true;
            },

            '0'..='9' => {
                let i = c as usize - '0' as usize;
                if !symbol.is_empty() {
                    result.push_str(symbol.repeat(i).as_str());
                    symbol.clear();
                }
                else {
                    return Err(Error::new(ErrorKind::InvalidInput, "Invalid string!"));
                }
            },

            _ => {
                if !symbol.is_empty() {
                    result.push_str(symbol.as_str());
                    symbol.clear();
                }
                symbol.push(c);
            }
        }
    }
    if !symbol.is_empty() {
        result.push_str(symbol.as_str());
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unpack_string() {
        assert_eq!(unpack_string(&"a4bc2d5e".to_string()).unwrap(), "aaaabccddddde");
        assert_eq!(unpack_string(&"abcd".to_string()).unwrap(), "abcd");
        assert_eq!(unpack_string(&"45".to_string()).is_err(), true);
        assert_eq!(unpack_string(&"".to_string()).unwrap(), "");
    }

    #[test]
    fn test_unpack_with_escape() {
        assert_eq!(unpack_string(&r"qwe\4\5".to_string()).unwrap(), r"qwe45");
        assert_eq!(unpack_string(&r"qwe\45".to_string()).unwrap(), r"qwe44444");
        assert_eq!(unpack_string(&r"qwe\\5".to_string()).unwrap(), r"qwe\\\\\");
    }

    #[test]
    fn test_incorrect_strings() {
        assert!(unpack_string(&"45".to_string()).is_err());
    }
}


fn main() {
    let str = String::from(r"qwe\\5");
    match unpack_string(&str) {
        Ok(s) => println!("{} => {}", str, s),
        Err(e) => println!("Error: {}", e),
    }
}

