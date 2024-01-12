use quote::{format_ident, quote};

pub fn main() -> anyhow::Result<()> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path("./ban-registry.txt")?;

    let mut country_code = Vec::<String>::new();
    let mut bban_structure = Vec::<String>::new();
    let mut bank_identifier_pos = Vec::<String>::new();
    let mut branch_identifier_pos = Vec::<String>::new();

    for (row, data) in reader.byte_records().enumerate() {
        match row {
            1 => {
                country_code = data?
                    .iter()
                    .skip(1)
                    .map(|b| String::from_utf8(b.to_vec()))
                    .collect::<Result<Vec<_>, _>>()?;
            }
            7 => {
                bban_structure = data?
                    .iter()
                    .skip(1)
                    .map(|b| String::from_utf8(b.to_vec()))
                    .collect::<Result<Vec<_>, _>>()?;
            }
            9 => {
                bank_identifier_pos = data?
                    .iter()
                    .skip(1)
                    .map(|b| String::from_utf8(b.to_vec()))
                    .collect::<Result<Vec<_>, _>>()?;
            }
            11 => {
                branch_identifier_pos = data?
                    .iter()
                    .skip(1)
                    .map(|b| String::from_utf8(b.to_vec()))
                    .collect::<Result<Vec<_>, _>>()?;
            }
            _ => {}
        }
    }

    let x = std::iter::zip(country_code, bban_structure);
    let y = std::iter::zip(bank_identifier_pos, branch_identifier_pos);

    let res = std::iter::zip(x, y);

    let re = regex::Regex::new("\\d+![anc]")?;

    let mut country_variants = quote! {};
    let mut country_parse = quote! {};
    let mut country_display = quote! {};
    let mut bank_ident = quote! {};
    let mut branch_ident = quote! {};
    let mut bban_format = quote! {};

    for ((country_code, bban_structure), (bank_ident_pos, branch_ident_pos)) in res {
        let country_ident = format_ident!("{country_code}");

        println!("{country_code} => {bban_structure}");

        let mut bban_sets = quote! {};
        for m in re.find_iter(&bban_structure) {
            let set = m.as_str();
            let set = set.split('!').collect::<Vec<_>>();
            let count: usize = set[0].parse()?;
            let charset = set[1];

            println!("  count {count}, charset {charset}");

            let charset = match charset {
                "a" => quote! { crate::bban::CharacterSet::Alphabetic },
                "n" => quote! { crate::bban::CharacterSet::Numeric },
                "c" => quote! { crate::bban::CharacterSet::Alphanumeric },
                _ => panic!("unsupported"),
            };

            bban_sets = quote! {
                #bban_sets
                #charset(#count),
            }
        }

        let bank_ident_pos = bank_ident_pos.replace('\n', "");
        let bank_ident_split = bank_ident_pos.split('-').collect::<Vec<_>>();
        if bank_ident_split.len() == 2 {
            let from: usize = bank_ident_split[0].parse()?;
            let to: usize = bank_ident_split[1].parse()?;
            bank_ident = quote! {
                #bank_ident
                Self::#country_ident => Some(#from..=#to),
            };
        }

        let branch_ident_pos = branch_ident_pos.replace('\n', "");
        let branch_ident_split = branch_ident_pos.split('-').collect::<Vec<_>>();
        if branch_ident_split.len() == 2 {
            let from: usize = branch_ident_split[0].parse()?;
            let to: usize = branch_ident_split[1].parse()?;
            branch_ident = quote! {
                #branch_ident
                Self::#country_ident => Some(#from..=#to),
            };
        }

        country_variants = quote! {
            #country_variants
            #country_ident,
        };
        country_parse = quote! {
            #country_parse
            #country_code => Ok(Self::#country_ident),
        };
        country_display = quote! {
            #country_display
            Self::#country_ident => write!(f, #country_code),
        };
        bban_format = quote! {
            #bban_format
            Self::#country_ident => &[#bban_sets],
        }
    }

    let countries_rs = quote::quote! {
        #[derive(Debug, PartialEq)]
        pub enum Country {
            #country_variants
        }

        impl Country {
            pub fn bank_identifier_pos(&self) -> Option<std::ops::RangeInclusive<usize>> {
                let pos = match self {
                    #bank_ident
                    _ => None,
                };

                pos.map(|r| {
                    let s = r.start() - 1;
                    let r = r.end() - 1;
                    s..=r
                })
            }

            pub fn branch_identifier_pos(&self) -> Option<std::ops::RangeInclusive<usize>> {
                let pos = match self {
                    #branch_ident
                    _ => None,
                };

                pos.map(|r| {
                    let s = r.start() - 1;
                    let r = r.end() - 1;
                    s..=r
                })
            }
        }

        impl std::str::FromStr for Country {
            type Err = crate::IbanError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #country_parse
                    _ => Err(crate::IbanError::UnsupportedCountry),
                }
            }
        }

        impl std::fmt::Display for Country {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #country_display
                }
            }
        }

        impl crate::bban::BbanFormat for Country {
            fn bban_format(&self) -> &'static [crate::bban::CharacterSet] {
                match self {
                    #bban_format
                }
            }
        }

        impl crate::bban::RandomBban for Country {
            fn rand(&self) -> String {
                use crate::bban::BbanFormat;

                self.bban_format().rand()
            }
        }
    };

    std::fs::write("src/countries.rs", countries_rs.to_string())?;

    std::process::Command::new("rustfmt")
        .arg("src/countries.rs")
        .output()
        .expect("Unable to handle process");

    Ok(())
}
