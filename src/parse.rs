use crate::annotations::Annotation;
use crate::git::Section;
use markdown_it::{plugins::cmark, MarkdownIt, Node};
use unidiff::PatchSet;

pub fn annotate(sections: impl Iterator<Item = Section>) -> impl Iterator<Item = Section> {
  sections.map(|mut section| {
    section.text.insert_str(0, &"\n");
    section
      .text
      .insert_str(0, &(Annotation::section_begin(section.id).value + "\n"));

    section
      .text
      .push_str(&Annotation::patchset(section.id).value);
    section.text.push('\n');

    section
      .text
      .push_str(&Annotation::section_end(section.id).value);
    section.text.push('\n');

    section
  })
}

pub fn concat(sections: impl Iterator<Item = Section>) -> (String, PatchSetsCollection) {
  let (text, patchsets) = sections.fold(
    (String::new(), vec![]),
    |(mut text, mut patchsets), section| {
      patchsets.push(section.patchset);
      text.push_str(&section.text);
      (text, patchsets)
    },
  );
  (text, PatchSetsCollection(patchsets))
}

#[derive(Debug)]
pub struct PatchSetsCollection(Vec<PatchSet>);

pub fn build_parser() -> MarkdownIt {
  let mut parser = MarkdownIt::new();

  cmark::add(&mut parser);

  parser
}

pub fn parse(text: String) -> Node {
  let parser = build_parser();

  parser.parse(&text)
}
