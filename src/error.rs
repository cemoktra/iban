use thiserror::Error;

#[derive(Debug, Error)]
pub enum IbanError {
    #[error("Country not supported")]
    UnsupportedCountry,
    #[error("BBAN is invalid")]
    InvalidBBAN,
    #[error("Check digit validation failed")]
    InvalidCheckDigits,
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}
