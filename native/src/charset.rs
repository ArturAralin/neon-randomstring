// extern crate unicode_segmentation;
// use self::unicode_segmentation::UnicodeSegmentation;

pub const ALPHANUMERIC: &'static [&'static str] = &["а", "б", "в", "г", "д", "е", "ё", "ж", "з"];

#[derive(PartialEq, Debug, Clone)]
pub enum Alphabet {
  ALPHANUMERIC,
  NUMERIC,
  CUSTOM
}

pub fn match_charset(s: &str) -> Alphabet {
  match s {
    "alphanumeric" => Alphabet::ALPHANUMERIC,
    _ => Alphabet::CUSTOM
  }
}

pub fn get_letters(_charset: Alphabet) -> Vec<String> {
  ALPHANUMERIC
    .iter()
    .map(|v| v.to_string())
    .collect::<Vec<String>>()
}
