// TODO: this file can be generated by parsing https://www.swift.com/node/11971

use crate::bban::{BbanFormat, CharacterSet, RandomBban};
use crate::IbanError;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
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
    BI,
    BR,
    BY,

    CH,
    CR,
    CY,
    CZ,

    DE,
    DJ,
    DK,
    DO,

    EE,
    EG,
    ES,

    FI,
    FK,
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
    MN,
    MR,
    MT,
    MU,

    NI,
    NL,
    NO,

    PK,
    PL,
    PS,
    PT,

    QA,

    RO,
    RS,
    RU,

    SA,
    SC,
    SD,
    SE,
    SI,
    SK,
    SM,
    SO,
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

impl Country {
    pub fn bank_identifier_pos(&self) -> Option<std::ops::RangeInclusive<usize>> {
        let pos = match self {
            Country::AD => Some(1..=4),
            Country::AE => Some(1..=3),
            Country::AL => Some(1..=3),
            Country::AT => Some(1..=5),
            Country::AZ => Some(1..=4),
            Country::BA => Some(1..=3),
            Country::BE => Some(1..=3),
            Country::BG => Some(1..=4),
            Country::BH => Some(1..=4),
            Country::BI => Some(1..=5),
            Country::BR => Some(1..=8),
            Country::BY => Some(1..=4),
            Country::CH => Some(1..=5),
            Country::CR => Some(1..=4),
            Country::CY => Some(1..=3),
            Country::CZ => Some(1..=4),
            Country::DE => Some(1..=8),
            Country::DJ => Some(1..=5),
            Country::DK => Some(1..=4),
            Country::DO => Some(1..=4),
            Country::EE => Some(1..=2),
            Country::EG => Some(1..=4),
            Country::ES => Some(1..=4),
            Country::FI => Some(1..=3),
            Country::FK => Some(1..=2),
            Country::FO => Some(1..=4),
            Country::FR => Some(1..=5),
            Country::GB => Some(1..=4),
            Country::GE => Some(1..=2),
            Country::GI => Some(1..=4),
            Country::GL => Some(1..=4),
            Country::GR => Some(1..=3),
            Country::GT => Some(1..=4),
            Country::HR => Some(1..=7),
            Country::HU => Some(1..=3),
            Country::IE => Some(1..=4),
            Country::IL => Some(1..=3),
            Country::IQ => Some(1..=4),
            Country::IS => Some(1..=2),
            Country::IT => Some(2..=6),
            Country::JO => Some(4..=8),
            Country::KW => Some(1..=4),
            Country::KZ => Some(1..=3),
            Country::LB => Some(1..=4),
            Country::LC => Some(1..=4),
            Country::LI => Some(1..=5),
            Country::LU => Some(1..=3),
            Country::LV => Some(1..=4),
            Country::LY => Some(1..=3),
            Country::MC => Some(1..=5),
            Country::MD => Some(1..=2),
            Country::ME => Some(1..=3),
            Country::MK => Some(1..=3),
            Country::MN => Some(1..=4),
            Country::MR => Some(1..=5),
            Country::MT => Some(1..=4),
            Country::MU => Some(1..=6),
            Country::NI => Some(1..=4),
            Country::NL => Some(1..=4),
            Country::NO => Some(1..=4),
            Country::PK => Some(1..=4),
            Country::PL => None,
            Country::PS => Some(1..=4),
            Country::PT => Some(1..=4),
            Country::QA => Some(1..=4),
            Country::RO => Some(1..=4),
            Country::RS => Some(1..=3),
            Country::RU => Some(1..=9),
            Country::SA => Some(1..=2),
            Country::SC => Some(1..=6),
            Country::SD => Some(1..=2),
            Country::SE => Some(1..=3),
            Country::SI => Some(1..=5),
            Country::SK => Some(1..=4),
            Country::SM => Some(2..=6),
            Country::SO => Some(1..=4),
            Country::ST => Some(1..=4),
            Country::SV => Some(1..=4),
            Country::TL => Some(1..=3),
            Country::TN => Some(1..=2),
            Country::TR => Some(1..=5),
            Country::UA => Some(1..=6),
            Country::VA => Some(1..=3),
            Country::VG => Some(1..=4),
            Country::XK => Some(1..=2),
        };

        pos.map(|r| {
            let s = r.start() - 1;
            let r = r.end() - 1;
            s..=r
        })
    }

    pub fn branch_identifier_pos(&self) -> Option<std::ops::RangeInclusive<usize>> {
        let pos = match self {
            Country::AD => Some(5..=8),
            Country::AL => Some(4..=8),
            Country::BA => Some(4..=6),
            Country::BG => Some(5..=8),
            Country::BI => Some(6..=10),
            Country::BR => Some(9..=13),
            Country::CY => Some(4..=8),
            Country::DJ => Some(6..=10),
            Country::EG => Some(5..=8),
            Country::ES => Some(5..=8),
            Country::GB => Some(5..=10),
            Country::GR => Some(4..=7),
            Country::HU => Some(4..=7),
            Country::IE => Some(5..=10),
            Country::IL => Some(4..=6),
            Country::IQ => Some(5..=7),
            Country::IS => Some(3..=4),
            Country::IT => Some(7..=11),
            Country::JO => Some(5..=8),
            Country::LY => Some(4..=6),
            Country::MC => Some(6..=10),
            Country::MR => Some(6..=10),
            Country::MT => Some(5..=9),
            Country::MU => Some(7..=8),
            Country::PL => Some(1..=8),
            Country::RU => Some(10..=14),
            Country::SC => Some(7..=8),
            Country::SM => Some(7..=11),
            Country::SO => Some(5..=7),
            Country::ST => Some(5..=8),
            Country::TN => Some(3..=5),
            Country::XK => Some(3..=4),
            _ => None,
        };

        pos.map(|r| {
            let s = r.start() - 1;
            let r = r.end() - 1;
            s..=r
        })
    }
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
            "BI" => Ok(Self::BI),
            "BR" => Ok(Self::BR),
            "BY" => Ok(Self::BY),

            "CH" => Ok(Self::CH),
            "CR" => Ok(Self::CR),
            "CY" => Ok(Self::CY),
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

            "NI" => Ok(Self::NI),
            "NL" => Ok(Self::NL),
            "NO" => Ok(Self::NO),

            "PK" => Ok(Self::PK),
            "PL" => Ok(Self::PL),
            "PS" => Ok(Self::PS),
            "PT" => Ok(Self::PT),

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
            _ => Err(IbanError::UnsupportedCountry),
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
            Country::BI => write!(f, "BI"),
            Country::BR => write!(f, "BR"),
            Country::BY => write!(f, "BY"),
            Country::CH => write!(f, "CH"),
            Country::CR => write!(f, "CR"),
            Country::CY => write!(f, "CY"),
            Country::CZ => write!(f, "CZ"),
            Country::DE => write!(f, "DE"),
            Country::DJ => write!(f, "DJ"),
            Country::DK => write!(f, "DK"),
            Country::DO => write!(f, "DO"),
            Country::EE => write!(f, "EE"),
            Country::EG => write!(f, "EG"),
            Country::ES => write!(f, "ES"),
            Country::FI => write!(f, "FI"),
            Country::FK => write!(f, "FK"),
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
            Country::MN => write!(f, "MN"),
            Country::MR => write!(f, "MR"),
            Country::MT => write!(f, "MT"),
            Country::MU => write!(f, "MU"),
            Country::NI => write!(f, "NI"),
            Country::NL => write!(f, "NL"),
            Country::NO => write!(f, "NO"),
            Country::PK => write!(f, "PK"),
            Country::PL => write!(f, "PL"),
            Country::PS => write!(f, "PS"),
            Country::PT => write!(f, "PT"),
            Country::QA => write!(f, "QA"),
            Country::RO => write!(f, "RO"),
            Country::RS => write!(f, "RS"),
            Country::RU => write!(f, "RU"),
            Country::SA => write!(f, "SA"),
            Country::SC => write!(f, "SC"),
            Country::SD => write!(f, "SD"),
            Country::SE => write!(f, "SE"),
            Country::SI => write!(f, "SI"),
            Country::SK => write!(f, "SK"),
            Country::SM => write!(f, "SM"),
            Country::SO => write!(f, "SO"),
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
            Country::BI => &[CharacterSet::Numeric(23)],
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
            Country::CY => &[CharacterSet::Numeric(8), CharacterSet::Alphanumeric(16)],
            Country::CZ => &[CharacterSet::Numeric(20)],

            Country::DE => &[CharacterSet::Numeric(18)],
            Country::DJ => &[CharacterSet::Numeric(23)],
            Country::DK => &[CharacterSet::Numeric(14)],
            Country::DO => &[CharacterSet::Alphanumeric(4), CharacterSet::Numeric(20)],

            Country::EE => &[CharacterSet::Numeric(16)],
            Country::EG => &[CharacterSet::Numeric(25)],
            Country::ES => &[CharacterSet::Numeric(20)],

            Country::FI => &[CharacterSet::Numeric(14)],
            Country::FK => &[CharacterSet::Alphabetic(2), CharacterSet::Numeric(12)],
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
            Country::MN => &[CharacterSet::Numeric(16)],
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

            Country::NI => &[CharacterSet::Alphabetic(4), CharacterSet::Numeric(20)],
            Country::NL => &[CharacterSet::Alphabetic(4), CharacterSet::Numeric(10)],
            Country::NO => &[CharacterSet::Numeric(11)],

            Country::PK => &[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(16)],
            Country::PL => &[CharacterSet::Numeric(24)],
            Country::PS => &[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(21)],
            Country::PT => &[CharacterSet::Numeric(21)],

            Country::QA => &[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(21)],

            Country::RO => &[CharacterSet::Alphabetic(4), CharacterSet::Alphanumeric(16)],
            Country::RS => &[CharacterSet::Numeric(18)],
            Country::RU => &[CharacterSet::Numeric(16), CharacterSet::Alphanumeric(15)],

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
            Country::SO => &[CharacterSet::Numeric(19)],
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
