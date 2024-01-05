use crate::bban::{BbanFormat, RandomBban, ValidateBban};
use crate::IbanError;
use iso_country::Country;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub struct Iban {
    country_code: Country,
    check_digits: u8,
    bban: String,
}

impl Iban {
    pub fn rand(country_code: Country) -> Result<Self, IbanError> {
        let bban = country_code.bban_format()?.rand();
        let mut slf = Self {
            country_code,
            check_digits: 0,
            bban,
        };

        let modulo = slf.modulo();
        println!("{modulo}");
        slf.check_digits = (98 - modulo) as u8;

        Ok(slf)
    }
    pub fn country(&self) -> Country {
        self.country_code
    }

    pub fn check_digits(&self) -> u8 {
        self.check_digits
    }

    pub fn bban(&self) -> &str {
        &self.bban
    }

    fn modulo(&self) -> u64 {
        let check_string = format!(
            "{}{:0>2}{}",
            &self.bban,
            &self.country_code.to_string(),
            self.check_digits
        );
        let digits = to_digits(&check_string);
        let mut count = 0u64;
        let mut n = 0u64;

        for digit in digits {
            n = n * 10 + digit as u64;
            count += 1;

            if count == 9 {
                n = n % 97;
                if n < 10 {
                    count = 1;
                } else {
                    count = 2;
                }
            }
        }

        n % 97
    }
}

fn to_digits(s: &str) -> Vec<u8> {
    s.chars().fold(Vec::new(), |mut acc, c| {
        if c.is_alphabetic() {
            let b = (c as u8) - 55;
            acc.push(b / 10);
            acc.push(b % 10);
        } else {
            if let Some(d) = c.to_digit(10) {
                acc.push(d as u8);
            }
        }
        acc
    })
}

impl Display for Iban {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let iban_str = format!(
            "{}{:0>2}{}",
            &self.country_code.to_string(),
            self.check_digits,
            &self.bban,
        );

        // TODO: human readable form
        write!(f, "{iban_str}")
    }
}

impl FromStr for Iban {
    type Err = IbanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bban = s.chars().filter(|c| !c.is_whitespace()).collect::<String>();
        let country_code = bban.drain(..2).collect::<String>();
        let check_digits = bban.drain(..2).collect::<String>();

        let country_code = Country::from_str(&country_code)?;
        let check_digits = check_digits.parse()?;

        country_code.bban_format()?.validate_bban(&bban)?;

        let iban = Self {
            country_code,
            check_digits,
            bban,
        };

        let modulo = iban.modulo();

        if modulo != 1 {
            return Err(IbanError::InvalidCheckDigits);
        }

        Ok(iban)
    }
}

#[cfg(test)]
mod test {
    use iso_country::Country;
    use std::str::FromStr;

    #[test]
    fn test_parse_iban() {
        let iban_str = "GB82 WEST 1234 5698 7654 32";
        let iban = super::Iban::from_str(iban_str).unwrap();

        assert_eq!(Country::GB, iban.country_code);
        assert_eq!(82, iban.check_digits);
        assert_eq!("WEST12345698765432", iban.bban);
    }

    #[test]
    fn test_random_iban() {
        let iban = super::Iban::rand(Country::GB).unwrap();
        let iban_str = iban.to_string();
        assert!(super::Iban::from_str(&iban_str).is_ok())
    }
}
