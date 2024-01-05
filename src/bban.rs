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
    fn bban_format(&self) -> Result<&'static [CharacterSet], IbanError>;
}

pub(crate) trait ValidateBban {
    fn validate_bban(&self, bban: &str) -> Result<(), IbanError>;
}

pub(crate) trait RandomBban {
    fn rand(&self) -> String;
}

impl BbanFormat for String {
    fn bban_format(&self) -> Result<&'static [CharacterSet], IbanError> {
        match self.as_str() {
            "AL" => Ok(&[CharacterSet::Numeric(8), CharacterSet::Alphanumeric(16)]),
            "AD" => Ok(&[CharacterSet::Numeric(8), CharacterSet::Alphanumeric(12)]),
            "AT" => Ok(&[CharacterSet::Numeric(16)]),
            "AZ" => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(20)]),
            "BH" => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(14)]),
            "BY" => Ok(&[
                CharacterSet::Alphanumeric(4),
                CharacterSet::Numeric(4),
                CharacterSet::Alphanumeric(16),
            ]),
            "BE" => Ok(&[CharacterSet::Numeric(12)]),
            "BA" => Ok(&[CharacterSet::Numeric(16)]),
            "BR" => Ok(&[
                CharacterSet::Numeric(23),
                CharacterSet::Alphabetic(1),
                CharacterSet::Alphanumeric(1),
            ]),
            "BG" => Ok(&[
                CharacterSet::Alphabetic(4),
                CharacterSet::Numeric(6),
                CharacterSet::Alphanumeric(8),
            ]),
            "CR" => Ok(&[CharacterSet::Numeric(18)]),
            "HR" => Ok(&[CharacterSet::Numeric(17)]),
            "CZ" => Ok(&[CharacterSet::Numeric(20)]),
            "DK" => Ok(&[CharacterSet::Numeric(14)]),
            "DO" => Ok(&[CharacterSet::Alphanumeric(4), CharacterSet::Numeric(20)]),
            "TL" => Ok(&[CharacterSet::Numeric(19)]),
            "EG" => Ok(&[CharacterSet::Numeric(25)]),
            "SV" => Ok(&[CharacterSet::Alphanumeric(4), CharacterSet::Numeric(20)]),
            "EE" => Ok(&[CharacterSet::Numeric(16)]),
            "FO" => Ok(&[CharacterSet::Numeric(14)]),
            "FI" => Ok(&[CharacterSet::Numeric(14)]),
            "FR" => Ok(&[
                CharacterSet::Numeric(10),
                CharacterSet::Alphanumeric(11),
                CharacterSet::Numeric(2),
            ]),
            "GE" => Ok(&[CharacterSet::Alphabetic(2), CharacterSet::Numeric(16)]),
            "DE" => Ok(&[CharacterSet::Numeric(18)]),
            "GI" => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(15)]),
            "GR" => Ok(&[CharacterSet::Numeric(7), CharacterSet::Alphanumeric(16)]),
            "GL" => Ok(&[CharacterSet::Numeric(14)]),
            "GT" => Ok(&[
                CharacterSet::Alphanumeric(4),
                CharacterSet::Alphanumeric(20),
            ]),
            "HU" => Ok(&[CharacterSet::Numeric(24)]),
            "IS" => Ok(&[CharacterSet::Numeric(22)]),
            "IQ" => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Numeric(15)]),
            "IE" => Ok(&[
                CharacterSet::Alphabetic(4),
                CharacterSet::Numeric(6),
                CharacterSet::Numeric(8),
            ]),
            "IL" => Ok(&[CharacterSet::Numeric(19)]),
            "IT" => Ok(&[
                CharacterSet::Alphabetic(1),
                CharacterSet::Numeric(10),
                CharacterSet::Alphanumeric(12),
            ]),
            "JO" => Ok(&[
                CharacterSet::Alphabetic(4),
                CharacterSet::Numeric(4),
                CharacterSet::Alphanumeric(18),
            ]),
            "KZ" => Ok(&[CharacterSet::Numeric(3), CharacterSet::Alphanumeric(13)]),
            // // iso_country does not know kosovo XK
            "XK" => Ok(&[
                CharacterSet::Numeric(4),
                CharacterSet::Numeric(10),
                CharacterSet::Numeric(2),
            ]),
            "KW" => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(22)]),
            "LV" => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(13)]),
            "LB" => Ok(&[CharacterSet::Numeric(4), CharacterSet::Alphanumeric(20)]),
            "LY" => Ok(&[CharacterSet::Numeric(21)]),
            "LI" => Ok(&[CharacterSet::Numeric(5), CharacterSet::Alphanumeric(12)]),
            "LU" => Ok(&[CharacterSet::Numeric(3), CharacterSet::Alphanumeric(13)]),
            "MT" => Ok(&[
                CharacterSet::Alphabetic(4),
                CharacterSet::Numeric(5),
                CharacterSet::Alphanumeric(18),
            ]),
            "MR" => Ok(&[CharacterSet::Numeric(23)]),
            "MU" => Ok(&[
                CharacterSet::Alphabetic(4),
                CharacterSet::Numeric(19),
                CharacterSet::Alphabetic(3),
            ]),
            "MC" => Ok(&[
                CharacterSet::Numeric(10),
                CharacterSet::Alphanumeric(11),
                CharacterSet::Numeric(2),
            ]),
            "MD" => Ok(&[
                CharacterSet::Alphanumeric(2),
                CharacterSet::Alphanumeric(18),
            ]),
            "ME" => Ok(&[CharacterSet::Numeric(18)]),
            "NL" => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Numeric(10)]),
            "MK" => Ok(&[
                CharacterSet::Numeric(3),
                CharacterSet::Alphanumeric(10),
                CharacterSet::Numeric(2),
            ]),
            "NO" => Ok(&[CharacterSet::Numeric(11)]),
            "PK" => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(16)]),
            "PS" => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(21)]),
            "PL" => Ok(&[CharacterSet::Numeric(24)]),
            "PT" => Ok(&[CharacterSet::Numeric(21)]),
            "QA" => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(21)]),
            "RO" => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(16)]),
            "PU" => Ok(&[CharacterSet::Numeric(14), CharacterSet::Alphanumeric(15)]),
            "LC" => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(24)]),
            "SM" => Ok(&[
                CharacterSet::Alphabetic(1),
                CharacterSet::Numeric(10),
                CharacterSet::Alphanumeric(12),
            ]),
            "ST" => Ok(&[CharacterSet::Numeric(21)]),
            "SA" => Ok(&[CharacterSet::Numeric(2), CharacterSet::Alphanumeric(18)]),
            "RS" => Ok(&[CharacterSet::Numeric(18)]),
            "SC" => Ok(&[
                CharacterSet::Alphabetic(4),
                CharacterSet::Numeric(20),
                CharacterSet::Alphabetic(3),
            ]),
            "SK" => Ok(&[CharacterSet::Numeric(24)]),
            "SI" => Ok(&[CharacterSet::Numeric(15)]),
            "ES" => Ok(&[CharacterSet::Numeric(20)]),
            "SD" => Ok(&[CharacterSet::Numeric(14)]),
            "SE" => Ok(&[CharacterSet::Numeric(20)]),
            "CH" => Ok(&[CharacterSet::Numeric(5), CharacterSet::Alphanumeric(12)]),
            "TN" => Ok(&[CharacterSet::Numeric(20)]),
            "TR" => Ok(&[
                CharacterSet::Numeric(5),
                CharacterSet::Fixed('0'),
                CharacterSet::Alphanumeric(16),
            ]),
            "UA" => Ok(&[CharacterSet::Numeric(6), CharacterSet::Alphanumeric(19)]),
            "AE" => Ok(&[CharacterSet::Numeric(6), CharacterSet::Numeric(16)]),
            "GB" => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Numeric(14)]),
            "VA" => Ok(&[CharacterSet::Numeric(3), CharacterSet::Numeric(15)]),
            "VG" => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Numeric(16)]),
            _ => Err(IbanError::UnsupportedCountry),
        }
    }
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
