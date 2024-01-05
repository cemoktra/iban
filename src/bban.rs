use crate::IbanError;
use iso_country::Country;
use rand::prelude::SliceRandom;

#[derive(Debug)]
pub(crate) enum CharacterSet {
    // a
    Alphabetic(usize),
    // c
    Alphanumeric(usize),
    // n
    Numeric(usize),
}

impl CharacterSet {
    fn chars(&self) -> usize {
        match self {
            CharacterSet::Alphabetic(l) => *l,
            CharacterSet::Alphanumeric(l) => *l,
            CharacterSet::Numeric(l) => *l,
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
        }
    }
}

pub(crate) trait BbanFormat {
    fn bban_format(&self) -> Result<&'static [CharacterSet], IbanError>;
}

pub(crate) trait ValidateBban {
    fn validate_bban(&self, bban: &str) -> Result<(), IbanError>;
}

pub(crate) trait RandomBban {
    fn rand(&self) -> String;
}

impl BbanFormat for Country {
    fn bban_format(&self) -> Result<&'static [CharacterSet], IbanError> {
        match self {
            Country::DE => Ok(&[CharacterSet::Numeric(18)]),
            Country::DK => Ok(&[CharacterSet::Numeric(14)]),
            Country::GB => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Numeric(14)]),
            Country::LU => Ok(&[CharacterSet::Numeric(3), CharacterSet::Alphanumeric(13)]),
            _ => Err(IbanError::UnsupportedCountry),
        }
    }
}

impl ValidateBban for &[CharacterSet] {
    fn validate_bban(&self, bban: &str) -> Result<(), IbanError> {
        let mut fmt_iter = self.iter();
        let mut current_fmt = fmt_iter.next().ok_or(IbanError::InvalidBBAN)?;
        let mut offset = 0;
        let mut char_iter = bban.chars();
        let mut char = char_iter.next();

        while let Some(c) = char {
            if c.is_whitespace() {
                char = char_iter.next();
                continue;
            }

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
            match set {
                CharacterSet::Alphabetic(l) => bban.push_str(
                    &[
                        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
                        'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
                    ]
                    .choose_multiple(&mut rng, *l)
                    .collect::<String>(),
                ),

                CharacterSet::Alphanumeric(l) => bban.push_str(
                    &[
                        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
                        'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3',
                        '4', '5', '6', '7', '8', '9',
                    ]
                    .choose_multiple(&mut rng, *l)
                    .collect::<String>(),
                ),
                CharacterSet::Numeric(l) => bban.push_str(
                    &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
                        .choose_multiple(&mut rng, *l)
                        .collect::<String>(),
                ),
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
