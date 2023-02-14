use once_cell::sync::Lazy;
use regex::Regex;

const SECTION_BEGIN: &str = r#"SECTION_BEGIN"#;
const SECTION_END: &str = r#"SECTION_END"#;
const PATCHSET: &str = r#"PATCHSET"#;

const MAGIC: &str = r#"@@@"#;
const SEP: &str = r#"~"#;

const HASH_GROUP_REGEX: &str = "(?P<section_hash>[a-zA-Z0-9]+)";

#[derive(Debug, Clone, PartialEq)]
pub struct Marker {
  pub value: String,
}

fn build(values: &[&str]) -> String {
  values
    .iter()
    .chain(Some(&MAGIC))
    .fold(MAGIC.to_owned(), |a, b| a + SEP + b)
}

impl Marker {
  pub fn section_begin(hash: &str) -> Self {
    Self {
      value: build(&[SECTION_BEGIN, hash]),
    }
  }

  pub fn section_end(hash: &str) -> Self {
    Self {
      value: build(&[SECTION_END, hash]),
    }
  }

  pub fn patchset(hash: &str) -> Self {
    Self {
      value: build(&[PATCHSET, hash]),
    }
  }

  pub fn section_begin_regex() -> &'static Lazy<Regex> {
    &SECTION_BEGIN_REGEX
  }

  pub fn section_end_regex() -> &'static Lazy<Regex> {
    &SECTION_END_REGEX
  }

  pub fn patchset_regex() -> &'static Lazy<Regex> {
    &PATCHSET_REGEX
  }
}

static SECTION_BEGIN_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(&build(&[SECTION_BEGIN, HASH_GROUP_REGEX])).unwrap());

static SECTION_END_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(&build(&[SECTION_END, HASH_GROUP_REGEX])).unwrap());

static PATCHSET_REGEX: Lazy<Regex> =
  Lazy::new(|| Regex::new(&build(&[PATCHSET, HASH_GROUP_REGEX])).unwrap());
