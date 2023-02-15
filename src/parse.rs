use crate::{git, markers::Marker, patchset::add_patchset_rules, section::add_section_rules};
use markdown_it::{plugins::cmark, MarkdownIt, Node};

pub fn annotate(
  sections: impl Iterator<Item = git::Section>,
) -> impl Iterator<Item = git::Section> {
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

pub fn concat(sections: impl Iterator<Item = git::Section>) -> String {
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

pub fn parse(tree: String) -> Node {
  let parser = build_parser();

  let log = git::log(&tree);
  let sections = log.sections();
  let annotated_sections = annotate(sections);
  let text = concat(annotated_sections);

  parser.parse(&text)
}
