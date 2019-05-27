#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidCode,
    CodeRangeExceeded,
}

const BASE: usize = 62;
const CHARSET: [char; BASE] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z',
];
const DIGI_OFFSET: usize = 48;
const UPPERCASE_OFFSET: usize = 55;
const LOWERCASE_OFFSET: usize = 61;
const MAX_CODE: &str = "LygHa16AHYF";
const MAX_CODE_LEN: usize = 11;

fn char_to_index(c: char) -> Result<usize, Error> {
    if c >= '0' && c <= '9' {
        return Ok((c as usize) - DIGI_OFFSET);
    } else if c >= 'A' && c <= 'Z' {
        return Ok((c as usize) - UPPERCASE_OFFSET);
    } else if c >= 'a' && c <= 'z' {
        return Ok((c as usize) - LOWERCASE_OFFSET);
    }
    Err(Error::InvalidCode)
}

/// encode number into base64 string
pub fn encode(n: u64) -> String {
    if n == 0 {
        return String::from("0");
    }

    let base = BASE as u64;
    let mut chs: Vec<char> = vec![];
    let mut remained = n;

    while remained > 0 {
        let idx = (remained % base) as usize;
        chs.push(CHARSET[idx]);
        remained = remained / base;
    }

    chs.reverse();
    chs.into_iter().collect()
}

/// decode base64 string into number
///
/// # Errors
///
/// if code exceed it's limit return Err(Error::CodeRangeExceeded)
/// if there is invalid characters in code return Err(Error::InvalidCode)
///
pub fn decode(code: &String) -> Result<u64, Error> {
    let mut nums: Vec<usize> = vec![];
    for c in code.chars() {
        nums.push(char_to_index(c)?);
    }

    let code_len = code.chars().count();
    if code_len > MAX_CODE_LEN || (code_len == MAX_CODE_LEN && &code[..] > MAX_CODE) {
        return Err(Error::CodeRangeExceeded);
    }

    let base = BASE as u64;
    let mut result = 0;
    for n in nums {
        result *= base;
        result += n as u64;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        struct TItem {
            input: u64,
            output: &'static str,
        }
        let t_set = vec![
            TItem {
                input: 0,
                output: "0",
            },
            TItem {
                input: 42,
                output: "g",
            },
            TItem {
                input: 32590299105,
                output: "ZZZZZZ",
            },
            TItem {
                input: 56800235583,
                output: "zzzzzz",
            },
            TItem {
                input: 4294967295,
                output: "4gfFC3",
            },
            TItem {
                input: 18446744073709551615,
                output: "LygHa16AHYF",
            },
        ];
        for item in &t_set {
            assert!(encode((*item).input) == item.output);
        }
    }

    #[test]
    fn test_decode() {
        struct TItem {
            input: &'static str,
            output: Result<u64, Error>,
        }
        let t_set = vec![
            TItem {
                input: "0",
                output: Ok(0),
            },
            TItem {
                input: "g",
                output: Ok(42),
            },
            TItem {
                input: "ZZZZZZ",
                output: Ok(32590299105),
            },
            TItem {
                input: "zzzzzz",
                output: Ok(56800235583),
            },
            TItem {
                input: "4gfFC3",
                output: Ok(4294967295),
            },
            TItem {
                input: "LygHa16AHYF",
                output: Ok(18446744073709551615),
            },
            TItem {
                input: "MygHa16AHYF",
                output: Err(Error::CodeRangeExceeded),
            },
            TItem {
                input: "1LygHa16AHYF",
                output: Err(Error::CodeRangeExceeded),
            },
            TItem {
                input: "0-0",
                output: Err(Error::InvalidCode),
            },
            TItem {
                input: "O口0",
                output: Err(Error::InvalidCode),
            },
        ];
        for item in &t_set {
            assert_eq!(decode(&String::from(item.input)), item.output);
        }
    }
}
