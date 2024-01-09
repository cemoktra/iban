// TODO: this file can be generated by parsing https://www.swift.com/node/11971

use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::bban::{BbanFormat, CharacterSet, RandomBban};
use crate::IbanError;

pub enum Country {
    AD,
    AE,
    AL,
    AT,
    AZ,

    BA,
    BE,
    BG,
    BH,
    BR,
    BY,

    CH,
    CR,
    CZ,

    DE,
    DK,
    DO,

    EE,
    EG,
    ES,

    FI,
    FO,
    FR,

    GB,
    GE,
    GI,
    GL,
    GR,
    GT,

    HR,
    HU,

    IE,
    IL,
    IQ,
    IS,
    IT,

    JO,

    KW,
    KZ,

    LB,
    LC,
    LI,
    LU,
    LV,
    LY,

    MC,
    MD,
    ME,
    MK,
    MR,
    MT,
    MU,

    NL,
    NO,

    PK,
    PL,
    PS,
    PT,
    PU,

    RO,
    RS,

    QA,

    SA,
    SC,
    SD,
    SE,
    SI,
    SK,
    SM,
    ST,
    SV,

    TL,
    TN,
    TR,

    UA,

    VA,
    VG,

    XK,
}

impl FromStr for Country {
    type Err = IbanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AD" => Ok(Self::AD),
            "AE" => Ok(Self::AE),
            "AL" => Ok(Self::AL),
            "AT" => Ok(Self::AT),
            "AZ" => Ok(Self::AZ),

            "BA" => Ok(Self::BA),
            "BE" => Ok(Self::BE),
            "BG" => Ok(Self::BG),
            "BH" => Ok(Self::BH),
            "BR" => Ok(Self::BR),
            "BY" => Ok(Self::BY),

            "CH" => Ok(Self::CH),
            "CR" => Ok(Self::CR),
            "CZ" => Ok(Self::CZ),

            "DE" => Ok(Self::DE),
            "DK" => Ok(Self::DK),
            "DO" => Ok(Self::DO),

            "EE" => Ok(Self::EE),
            "EG" => Ok(Self::EG),
            "ES" => Ok(Self::ES),

            "FI" => Ok(Self::FI),
            "FO" => Ok(Self::FO),
            "FR" => Ok(Self::FR),

            "GB" => Ok(Self::GB),
            "GE" => Ok(Self::GE),
            "GI" => Ok(Self::GI),
            "GL" => Ok(Self::GL),
            "GR" => Ok(Self::GR),
            "GT" => Ok(Self::GT),

            "HR" => Ok(Self::HR),
            "HU" => Ok(Self::HU),

            "IE" => Ok(Self::IE),
            "IL" => Ok(Self::IL),
            "IQ" => Ok(Self::IQ),
            "IS" => Ok(Self::IS),
            "IT" => Ok(Self::IT),

            "JO" => Ok(Self::JO),

            "KW" => Ok(Self::KW),
            "KZ" => Ok(Self::KZ),

            "LB" => Ok(Self::LB),
            "LC" => Ok(Self::LC),
            "LI" => Ok(Self::LI),
            "LU" => Ok(Self::LU),
            "LV" => Ok(Self::LV),
            "LY" => Ok(Self::LY),

            "MC" => Ok(Self::MC),
            "MD" => Ok(Self::MD),
            "ME" => Ok(Self::ME),
            "MK" => Ok(Self::MK),
            "MR" => Ok(Self::MR),
            "MT" => Ok(Self::MT),
            "MU" => Ok(Self::MU),

            "NL" => Ok(Self::NL),
            "NO" => Ok(Self::NO),

            "PK" => Ok(Self::PK),
            "PL" => Ok(Self::PL),
            "PS" => Ok(Self::PS),
            "PT" => Ok(Self::PT),
            "PU" => Ok(Self::PU),

            "QA" => Ok(Self::QA),

            "RO" => Ok(Self::RO),
            "RS" => Ok(Self::RS),

            "SA" => Ok(Self::SA),
            "SC" => Ok(Self::SC),
            "SD" => Ok(Self::SD),
            "SE" => Ok(Self::SE),
            "SI" => Ok(Self::SI),
            "SK" => Ok(Self::SK),
            "SM" => Ok(Self::SM),
            "ST" => Ok(Self::ST),
            "SV" => Ok(Self::SV),

            "TL" => Ok(Self::TL),
            "TN" => Ok(Self::TN),
            "TR" => Ok(Self::TR),

            "UA" => Ok(Self::UA),

            "VA" => Ok(Self::VA),
            "VG" => Ok(Self::VG),

            "XK" => Ok(Self::XK),
            _ => Err(IbanError::UnsupportedCountry)
        }
    }
}

impl Display for Country {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Country::AD => write!(f, "AD"),
            Country::AE => write!(f, "AE"),
            Country::AL => write!(f, "AL"),
            Country::AT => write!(f, "AT"),
            Country::AZ => write!(f, "AZ"),
            Country::BA => write!(f, "BA"),
            Country::BE => write!(f, "BE"),
            Country::BG => write!(f, "BG"),
            Country::BH => write!(f, "BH"),
            Country::BR => write!(f, "BR"),
            Country::BY => write!(f, "BY"),
            Country::CH => write!(f, "CH"),
            Country::CR => write!(f, "CR"),
            Country::CZ => write!(f, "CZ"),
            Country::DE => write!(f, "DE"),
            Country::DK => write!(f, "DK"),
            Country::DO => write!(f, "DO"),
            Country::EE => write!(f, "EE"),
            Country::EG => write!(f, "EG"),
            Country::ES => write!(f, "ES"),
            Country::FI => write!(f, "FI"),
            Country::FO => write!(f, "FO"),
            Country::FR => write!(f, "FR"),
            Country::GB => write!(f, "GB"),
            Country::GE => write!(f, "GE"),
            Country::GI => write!(f, "GI"),
            Country::GL => write!(f, "GL"),
            Country::GR => write!(f, "GR"),
            Country::GT => write!(f, "GT"),
            Country::HR => write!(f, "HR"),
            Country::HU => write!(f, "HU"),
            Country::IE => write!(f, "IE"),
            Country::IL => write!(f, "IL"),
            Country::IQ => write!(f, "IQ"),
            Country::IS => write!(f, "IS"),
            Country::IT => write!(f, "IT"),
            Country::JO => write!(f, "JO"),
            Country::KW => write!(f, "KW"),
            Country::KZ => write!(f, "KZ"),
            Country::LB => write!(f, "LB"),
            Country::LC => write!(f, "LC"),
            Country::LI => write!(f, "LI"),
            Country::LU => write!(f, "LU"),
            Country::LV => write!(f, "LV"),
            Country::LY => write!(f, "LY"),
            Country::MC => write!(f, "MC"),
            Country::MD => write!(f, "MD"),
            Country::ME => write!(f, "ME"),
            Country::MK => write!(f, "MK"),
            Country::MR => write!(f, "MR"),
            Country::MT => write!(f, "MT"),
            Country::MU => write!(f, "MU"),
            Country::NL => write!(f, "NL"),
            Country::NO => write!(f, "NO"),
            Country::PK => write!(f, "PK"),
            Country::PL => write!(f, "PL"),
            Country::PS => write!(f, "PS"),
            Country::PT => write!(f, "PT"),
            Country::PU => write!(f, "PU"),
            Country::RO => write!(f, "RO"),
            Country::RS => write!(f, "RS"),
            Country::QA => write!(f, "QA"),
            Country::SA => write!(f, "SA"),
            Country::SC => write!(f, "SC"),
            Country::SD => write!(f, "SD"),
            Country::SE => write!(f, "SE"),
            Country::SI => write!(f, "SI"),
            Country::SK => write!(f, "SK"),
            Country::SM => write!(f, "SM"),
            Country::ST => write!(f, "ST"),
            Country::SV => write!(f, "SV"),
            Country::TL => write!(f, "TL"),
            Country::TN => write!(f, "TN"),
            Country::TR => write!(f, "TR"),
            Country::UA => write!(f, "UA"),
            Country::VA => write!(f, "VA"),
            Country::VG => write!(f, "VG"),
            Country::XK => write!(f, "XK"),
        }
    }
}

impl BbanFormat for Country {
    fn bban_format(&self) -> &'static [CharacterSet] {
        match self {
            Country::AD => &[CharacterSet::Numeric(8), CharacterSet::Alphanumeric(12)],
            Country::AE => &[CharacterSet::Numeric(6), CharacterSet::Numeric(16)],
            Country::AL => &[CharacterSet::Numeric(8), CharacterSet::Alphanumeric(16)],
            Country::AT => &[CharacterSet::Numeric(16)],
            Country::AZ => &[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(20)],

            Country::BA => &[CharacterSet::Numeric(16)],
            Country::BE => &[CharacterSet::Numeric(12)],
            Country::BG => &[
                CharacterSet::Alphabetic(4),
                CharacterSet::Numeric(6),
                CharacterSet::Alphanumeric(8),
            ],
            Country::BH => &[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(14)],
            Country::BR => &[
                CharacterSet::Numeric(23),
                CharacterSet::Alphabetic(1),
                CharacterSet::Alphanumeric(1),
            ],
            Country::BY => &[
                CharacterSet::Alphanumeric(4),
                CharacterSet::Numeric(4),
                CharacterSet::Alphanumeric(16),
            ],

            Country::CH => &[CharacterSet::Numeric(5), CharacterSet::Alphanumeric(12)],
            Country::CR => &[CharacterSet::Numeric(18)],
            Country::CZ => &[CharacterSet::Numeric(20)],

            Country::DE => &[CharacterSet::Numeric(18)],
            Country::DK => &[CharacterSet::Numeric(14)],
            Country::DO => &[CharacterSet::Alphanumeric(4), CharacterSet::Numeric(20)],

            Country::EE => &[CharacterSet::Numeric(16)],
            Country::EG => &[CharacterSet::Numeric(25)],
            Country::ES => &[CharacterSet::Numeric(20)],

            Country::FI => &[CharacterSet::Numeric(14)],
            Country::FO => &[CharacterSet::Numeric(14)],
            Country::FR => &[
                CharacterSet::Numeric(10),
                CharacterSet::Alphanumeric(11),
                CharacterSet::Numeric(2),
            ],

            Country::GB => &[CharacterSet::Alphabetic(4), CharacterSet::Numeric(14)],
            Country::GE => &[CharacterSet::Alphabetic(2), CharacterSet::Numeric(16)],
            Country::GI => &[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(15)],
            Country::GL => &[CharacterSet::Numeric(14)],
            Country::GR => &[CharacterSet::Numeric(7), CharacterSet::Alphanumeric(16)],
            Country::GT => &[
                CharacterSet::Alphanumeric(4),
                CharacterSet::Alphanumeric(20),
            ],

            Country::HR => &[CharacterSet::Numeric(17)],
            Country::HU => &[CharacterSet::Numeric(24)],

            Country::IE => &[
                CharacterSet::Alphabetic(4),
                CharacterSet::Numeric(6),
                CharacterSet::Numeric(8),
            ],
            Country::IL => &[CharacterSet::Numeric(19)],
            Country::IQ => &[CharacterSet::Alphabetic(4), CharacterSet::Numeric(15)],
            Country::IS => &[CharacterSet::Numeric(22)],
            Country::IT => &[
                CharacterSet::Alphabetic(1),
                CharacterSet::Numeric(10),
                CharacterSet::Alphanumeric(12),
            ],

            Country::JO => &[
                CharacterSet::Alphabetic(4),
                CharacterSet::Numeric(4),
                CharacterSet::Alphanumeric(18),
            ],

            Country::KW => &[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(22)],
            Country::KZ => &[CharacterSet::Numeric(3), CharacterSet::Alphanumeric(13)],

            Country::LB => &[CharacterSet::Numeric(4), CharacterSet::Alphanumeric(20)],
            Country::LC => &[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(24)],
            Country::LI => &[CharacterSet::Numeric(5), CharacterSet::Alphanumeric(12)],
            Country::LU => &[CharacterSet::Numeric(3), CharacterSet::Alphanumeric(13)],
            Country::LV => &[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(13)],
            Country::LY => &[CharacterSet::Numeric(21)],

            Country::MC => &[
                CharacterSet::Numeric(10),
                CharacterSet::Alphanumeric(11),
                CharacterSet::Numeric(2),
            ],
            Country::MD => &[
                CharacterSet::Alphanumeric(2),
                CharacterSet::Alphanumeric(18),
            ],
            Country::ME => &[CharacterSet::Numeric(18)],
            Country::MK => &[
                CharacterSet::Numeric(3),
                CharacterSet::Alphanumeric(10),
                CharacterSet::Numeric(2),
            ],
            Country::MR => &[CharacterSet::Numeric(23)],
            Country::MT => &[
                CharacterSet::Alphabetic(4),
                CharacterSet::Numeric(5),
                CharacterSet::Alphanumeric(18),
            ],
            Country::MU => &[
                CharacterSet::Alphabetic(4),
                CharacterSet::Numeric(19),
                CharacterSet::Alphabetic(3),
            ],

            Country::NL => &[CharacterSet::Alphabetic(4), CharacterSet::Numeric(10)],
            Country::NO => &[CharacterSet::Numeric(11)],

            Country::PK => &[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(16)],
            Country::PL => &[CharacterSet::Numeric(24)],
            Country::PS => &[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(21)],
            Country::PT => &[CharacterSet::Numeric(21)],
            Country::PU => &[CharacterSet::Numeric(14), CharacterSet::Alphanumeric(15)],

            Country::QA => &[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(21)],

            Country::RO => &[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(16)],
            Country::RS => &[CharacterSet::Numeric(18)],

            Country::SA => &[CharacterSet::Numeric(2), CharacterSet::Alphanumeric(18)],
            Country::SC => &[
                CharacterSet::Alphabetic(4),
                CharacterSet::Numeric(20),
                CharacterSet::Alphabetic(3),
            ],
            Country::SD => &[CharacterSet::Numeric(14)],
            Country::SE => &[CharacterSet::Numeric(20)],
            Country::SI => &[CharacterSet::Numeric(15)],
            Country::SK => &[CharacterSet::Numeric(24)],
            Country::SM => &[
                CharacterSet::Alphabetic(1),
                CharacterSet::Numeric(10),
                CharacterSet::Alphanumeric(12),
            ],
            Country::ST => &[CharacterSet::Numeric(21)],

            Country::SV => &[CharacterSet::Alphanumeric(4), CharacterSet::Numeric(20)],

            Country::TL => &[CharacterSet::Numeric(19)],
            Country::TN => &[CharacterSet::Numeric(20)],
            Country::TR => &[
                CharacterSet::Numeric(5),
                CharacterSet::Fixed('0'),
                CharacterSet::Alphanumeric(16),
            ],

            Country::UA => &[CharacterSet::Numeric(6), CharacterSet::Alphanumeric(19)],

            Country::VA => &[CharacterSet::Numeric(3), CharacterSet::Numeric(15)],
            Country::VG => &[CharacterSet::Alphabetic(4), CharacterSet::Numeric(16)],

            Country::XK => &[
                CharacterSet::Numeric(4),
                CharacterSet::Numeric(10),
                CharacterSet::Numeric(2),
            ],
        }
    }
}

impl RandomBban for Country {
    fn rand(&self) -> String {
        self.bban_format().rand()
    }
}