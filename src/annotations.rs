pub const SECTION_BEGIN: &str = r#"SECTION_BEGIN"#;
pub const SECTION_END: &str = r#"SECTION_END"#;
pub const PATCHSET: &str = r#"PATCHSET"#;

pub const MARKER: &str = r#"@@@"#;
pub const SEP: &str = r#"~"#;

#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
  pub value: String,
}

impl Annotation {
  pub fn new(value: &str) -> Self {
    Self {
      value: build(&[value]),
    }
  }

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
}

fn build(values: &[&str]) -> String {
  values
    .iter()
    .chain(Some(&MARKER))
    .fold(MARKER.to_owned(), |a, b| a + SEP + b)
}
