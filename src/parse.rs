use crate::{git::Section, markers::Marker, section::add_section_rules, patchset::add_patchset_rules};
use markdown_it::{plugins::cmark, MarkdownIt, Node};

pub fn annotate(sections: impl Iterator<Item = Section>) -> impl Iterator<Item = Section> {
  sections.map(|mut section| {
    section.text.insert_str(0, &"\n");
    section
      .text
      .insert_str(0, &(Marker::section_begin(&section.commit).value + "\n"));

    section
      .text
      .push_str(&Marker::patchset(&section.commit).value);
    section.text.push('\n');

    section
      .text
      .push_str(&Marker::section_end(&section.commit).value);
    section.text.push('\n');

    section
  })
}

pub fn concat(sections: impl Iterator<Item = Section>) -> String {
  let (text, _patchsets) = sections.fold(
    (String::new(), vec![]),
    |(mut text, mut patchsets), section| {
      patchsets.push((section.commit, section.subject, section.patchset));
      text.push_str(&section.text);
      (text, patchsets)
    },
  );
  text
}

pub fn build_parser() -> MarkdownIt {
  let mut parser = MarkdownIt::new();

  add_section_rules(&mut parser);
  add_patchset_rules(&mut parser);
  cmark::add(&mut parser);

  parser
}

pub fn parse(text: String) -> Node {
  let parser = build_parser();

  parser.parse(&text)
}
