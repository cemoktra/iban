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

impl BbanFormat for Country {
    fn bban_format(&self) -> Result<&'static [CharacterSet], IbanError> {
        match self {
            Country::AL => Ok(&[CharacterSet::Numeric(8), CharacterSet::Alphanumeric(16)]),
            Country::AD => Ok(&[CharacterSet::Numeric(8), CharacterSet::Alphanumeric(12)]),
            Country::AT => Ok(&[CharacterSet::Numeric(16)]),
            Country::AZ => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(20)]),
            Country::BH => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(14)]),
            Country::BY => Ok(&[
                CharacterSet::Alphanumeric(4),
                CharacterSet::Numeric(4),
                CharacterSet::Alphanumeric(16),
            ]),
            Country::BE => Ok(&[CharacterSet::Numeric(12)]),
            Country::BA => Ok(&[CharacterSet::Numeric(16)]),
            Country::BR => Ok(&[
                CharacterSet::Numeric(23),
                CharacterSet::Alphabetic(1),
                CharacterSet::Alphanumeric(1),
            ]),
            Country::BG => Ok(&[
                CharacterSet::Alphabetic(4),
                CharacterSet::Numeric(6),
                CharacterSet::Alphanumeric(8),
            ]),
            Country::CR => Ok(&[CharacterSet::Numeric(18)]),
            Country::HR => Ok(&[CharacterSet::Numeric(17)]),
            Country::CZ => Ok(&[CharacterSet::Numeric(20)]),
            Country::DK => Ok(&[CharacterSet::Numeric(14)]),
            Country::DO => Ok(&[CharacterSet::Alphanumeric(4), CharacterSet::Numeric(20)]),
            Country::TL => Ok(&[CharacterSet::Numeric(19)]),
            Country::EG => Ok(&[CharacterSet::Numeric(25)]),
            Country::SV => Ok(&[CharacterSet::Alphanumeric(4), CharacterSet::Numeric(20)]),
            Country::EE => Ok(&[CharacterSet::Numeric(16)]),
            Country::FO => Ok(&[CharacterSet::Numeric(14)]),
            Country::FI => Ok(&[CharacterSet::Numeric(14)]),
            Country::FR => Ok(&[
                CharacterSet::Numeric(10),
                CharacterSet::Alphanumeric(11),
                CharacterSet::Numeric(2),
            ]),
            Country::GE => Ok(&[CharacterSet::Alphabetic(2), CharacterSet::Numeric(16)]),
            Country::DE => Ok(&[CharacterSet::Numeric(18)]),
            Country::GI => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(15)]),
            Country::GR => Ok(&[CharacterSet::Numeric(7), CharacterSet::Alphanumeric(16)]),
            Country::GL => Ok(&[CharacterSet::Numeric(14)]),
            Country::GT => Ok(&[
                CharacterSet::Alphanumeric(4),
                CharacterSet::Alphanumeric(20),
            ]),
            Country::HU => Ok(&[CharacterSet::Numeric(24)]),
            Country::IS => Ok(&[CharacterSet::Numeric(22)]),
            Country::IQ => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Numeric(15)]),
            Country::IE => Ok(&[
                CharacterSet::Alphabetic(4),
                CharacterSet::Numeric(6),
                CharacterSet::Numeric(8),
            ]),
            Country::IL => Ok(&[CharacterSet::Numeric(19)]),
            Country::IT => Ok(&[
                CharacterSet::Alphabetic(1),
                CharacterSet::Numeric(10),
                CharacterSet::Alphanumeric(12),
            ]),
            Country::JO => Ok(&[
                CharacterSet::Alphabetic(4),
                CharacterSet::Numeric(4),
                CharacterSet::Alphanumeric(18),
            ]),
            Country::KZ => Ok(&[CharacterSet::Numeric(3), CharacterSet::Alphanumeric(13)]),
            // iso_country does not know kosovo XK
            //            Country::XK => Ok(&[
            //                CharacterSet::Numeric(4),
            //                CharacterSet::Numeric(10),
            //                CharacterSet::Numeric(2),
            //            ]),
            Country::KW => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(22)]),
            Country::LV => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(13)]),
            Country::LB => Ok(&[CharacterSet::Numeric(4), CharacterSet::Alphanumeric(20)]),
            Country::LY => Ok(&[CharacterSet::Numeric(21)]),
            Country::LI => Ok(&[CharacterSet::Numeric(5), CharacterSet::Alphanumeric(12)]),
            Country::LU => Ok(&[CharacterSet::Numeric(3), CharacterSet::Alphanumeric(13)]),
            Country::MT => Ok(&[
                CharacterSet::Alphabetic(4),
                CharacterSet::Numeric(5),
                CharacterSet::Alphanumeric(18),
            ]),
            Country::MR => Ok(&[CharacterSet::Numeric(23)]),
            Country::MU => Ok(&[
                CharacterSet::Alphabetic(4),
                CharacterSet::Numeric(19),
                CharacterSet::Alphabetic(3),
            ]),
            Country::MC => Ok(&[
                CharacterSet::Numeric(10),
                CharacterSet::Alphanumeric(11),
                CharacterSet::Numeric(2),
            ]),
            Country::MD => Ok(&[
                CharacterSet::Alphanumeric(2),
                CharacterSet::Alphanumeric(18),
            ]),
            Country::ME => Ok(&[CharacterSet::Numeric(18)]),
            Country::NL => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Numeric(10)]),
            Country::MK => Ok(&[
                CharacterSet::Numeric(3),
                CharacterSet::Alphanumeric(10),
                CharacterSet::Numeric(2),
            ]),
            Country::NO => Ok(&[CharacterSet::Numeric(11)]),
            Country::PK => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(16)]),
            Country::PS => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(21)]),
            Country::PL => Ok(&[CharacterSet::Numeric(24)]),
            Country::PT => Ok(&[CharacterSet::Numeric(21)]),
            Country::QA => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(21)]),
            Country::RO => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(16)]),
            Country::RU => Ok(&[CharacterSet::Numeric(14), CharacterSet::Alphanumeric(15)]),
            Country::LC => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(24)]),
            Country::SM => Ok(&[
                CharacterSet::Alphabetic(1),
                CharacterSet::Numeric(10),
                CharacterSet::Alphanumeric(12),
            ]),
            Country::ST => Ok(&[CharacterSet::Numeric(21)]),
            Country::SA => Ok(&[CharacterSet::Numeric(2), CharacterSet::Alphanumeric(18)]),
            Country::RS => Ok(&[CharacterSet::Numeric(18)]),
            Country::SC => Ok(&[
                CharacterSet::Alphabetic(4),
                CharacterSet::Numeric(20),
                CharacterSet::Alphabetic(3),
            ]),
            Country::SK => Ok(&[CharacterSet::Numeric(24)]),
            Country::SI => Ok(&[CharacterSet::Numeric(15)]),
            Country::ES => Ok(&[CharacterSet::Numeric(20)]),
            Country::SD => Ok(&[CharacterSet::Numeric(14)]),
            Country::SE => Ok(&[CharacterSet::Numeric(20)]),
            Country::CH => Ok(&[CharacterSet::Numeric(5), CharacterSet::Alphanumeric(12)]),
            Country::TN => Ok(&[CharacterSet::Numeric(20)]),
            Country::TR => Ok(&[
                CharacterSet::Numeric(5),
                CharacterSet::Fixed('0'),
                CharacterSet::Alphanumeric(16),
            ]),
            Country::UA => Ok(&[CharacterSet::Numeric(6), CharacterSet::Alphanumeric(19)]),
            Country::AE => Ok(&[CharacterSet::Numeric(6), CharacterSet::Numeric(16)]),
            Country::GB => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Numeric(14)]),
            Country::VA => Ok(&[CharacterSet::Numeric(3), CharacterSet::Numeric(15)]),
            Country::VG => Ok(&[CharacterSet::Alphabetic(4), CharacterSet::Numeric(16)]),

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
