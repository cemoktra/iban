use crate::IbanError;
use rand::prelude::SliceRandom;

#[derive(Debug)]
pub(crate) enum CharacterSet {
    // a
    Alphabetic(usize),
    // c
    Alphanumeric(usize),
    // n
    Numeric(usize),
    Fixed(char),
}

impl CharacterSet {
    fn chars(&self) -> usize {
        match self {
            CharacterSet::Alphabetic(l) => *l,
            CharacterSet::Alphanumeric(l) => *l,
            CharacterSet::Numeric(l) => *l,
            CharacterSet::Fixed(_) => 1,
        }
    }

    fn validate(&self, c: char) -> Result<usize, IbanError> {
        match self {
            CharacterSet::Alphabetic(len) => {
                if !c.is_alphabetic() {
                    return Err(IbanError::InvalidBBAN);
                }
                Ok(*len)
            }
            CharacterSet::Alphanumeric(len) => {
                if !c.is_alphanumeric() {
                    return Err(IbanError::InvalidBBAN);
                }
                Ok(*len)
            }
            CharacterSet::Numeric(len) => {
                if !c.is_numeric() {
                    return Err(IbanError::InvalidBBAN);
                }
                Ok(*len)
            }
            CharacterSet::Fixed(f) => {
                if c != *f {
                    return Err(IbanError::InvalidBBAN);
                }
                Ok(1)
            }
        }
    }
}

pub(crate) trait BbanFormat {
    fn bban_format(&self) -> &'static [CharacterSet];
}

pub(crate) trait ValidateBban {
    fn validate_bban(&self, bban: &str) -> Result<(), IbanError>;
}

pub(crate) trait RandomBban {
    fn rand(&self) -> String;
}


impl ValidateBban for &[CharacterSet] {
    fn validate_bban(&self, bban: &str) -> Result<(), IbanError> {
        let mut fmt_iter = self.iter();
        let mut current_fmt = fmt_iter.next().ok_or(IbanError::InvalidBBAN)?;
        let mut offset = 0;
        let mut char_iter = bban
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.to_ascii_uppercase());
        let mut char = char_iter.next();

        while let Some(c) = char {
            let fmt_length = current_fmt.validate(c)?;

            offset += 1;
            char = char_iter.next();

            if offset == fmt_length && char.is_some() {
                offset = 0;
                current_fmt = fmt_iter.next().ok_or(IbanError::InvalidBBAN)?;
            }
        }

        match (char, fmt_iter.next(), offset == current_fmt.chars()) {
            (None, None, true) => Ok(()),
            _ => Err(IbanError::InvalidBBAN),
        }
    }
}

impl RandomBban for &[CharacterSet] {
    fn rand(&self) -> String {
        let mut bban = String::new();
        let mut rng = rand::thread_rng();

        for set in self.iter() {
            let (alphabet, chars) = match set {
                CharacterSet::Alphabetic(l) => (
                    [
                        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
                        'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
                    ]
                    .as_slice(),
                    l,
                ),

                CharacterSet::Alphanumeric(l) => (
                    [
                        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
                        'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3',
                        '4', '5', '6', '7', '8', '9',
                    ]
                    .as_slice(),
                    l,
                ),
                CharacterSet::Numeric(l) => (
                    ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].as_slice(),
                    l,
                ),
                CharacterSet::Fixed(c) => {
                    bban.push(*c);
                    continue;
                }
            };

            for _ in 0..*chars {
                bban.push(
                    *alphabet
                        .choose(&mut rng)
                        .expect("failed to choose random item"),
                );
            }
        }

        bban
    }
}

#[cfg(test)]
mod test {
    use crate::bban::{CharacterSet, ValidateBban};

    const BBAN_FORMAT: &[CharacterSet] = &[CharacterSet::Alphabetic(4), CharacterSet::Numeric(8)];

    #[test]
    fn test_bban_validation() {
        assert!(BBAN_FORMAT.validate_bban("ABCD12345678").is_ok());
        assert!(BBAN_FORMAT.validate_bban("ABCD 1234 5678").is_ok());

        assert!(BBAN_FORMAT.validate_bban("ABCD123456789").is_err());
        assert!(BBAN_FORMAT.validate_bban("ABCD1234567").is_err());
        assert!(BBAN_FORMAT.validate_bban("ABCD1234567A").is_err());
        assert!(BBAN_FORMAT.validate_bban("1BCD12345678").is_err());
    }
}
