use crate::{annotations::Annotation, git::Section};
use markdown_it::{
  parser::block::{BlockRule, BlockState},
  plugins::cmark,
  MarkdownIt, Node, NodeValue, Renderer,
};
use once_cell::sync::Lazy;
use regex::Regex;
use unidiff::PatchSet;

pub fn annotate(sections: impl Iterator<Item = Section>) -> impl Iterator<Item = Section> {
  sections.map(|mut section| {
    section.text.insert_str(0, &"\n");
    section.text.insert_str(
      0,
      &(Annotation::section_begin(&section.commit).value + "\n"),
    );

    section
      .text
      .push_str(&Annotation::patchset(&section.commit).value);
    section.text.push('\n');

    section
      .text
      .push_str(&Annotation::section_end(&section.commit).value);
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

  add_section_scanner(&mut parser);
  cmark::add(&mut parser);

  parser
}

pub fn parse(text: String) -> Node {
  let parser = build_parser();

  parser.parse(&text)
}

pub struct SectionScanner;

use crate::annotations::{MARKER, SECTION_BEGIN, SECTION_END, SEP};
static SECTION_BEGIN_RE: Lazy<Regex> = Lazy::new(|| {
  Regex::new(&format!(
    r"^{MARKER}{SEP}{SECTION_BEGIN}{SEP}(?P<section_hash>[a-zA-Z0-9]+){SEP}{MARKER}$"
  ))
  .unwrap()
});
static SECTION_END_RE: Lazy<Regex> = Lazy::new(|| {
  Regex::new(&format!(
    r"^{MARKER}{SEP}{SECTION_END}{SEP}(?P<section_hash>[a-zA-Z0-9]+){SEP}{MARKER}$"
  ))
  .unwrap()
});

impl BlockRule for SectionScanner {
  fn run(state: &mut BlockState) -> Option<(Node, usize)> {
    let line = state.get_line(state.line).trim();

    if let Some(captures) = SECTION_BEGIN_RE.captures(line) {
      let section_hash = captures.name("section_hash").unwrap().as_str().into();

      Some((Node::new(SectionStart { hash: section_hash }), 1))
    } else if SECTION_END_RE.is_match(line) {
      Some((Node::new(SectionEnd), 1))
    } else {
      return None;
    }
  }
}

pub fn add_section_scanner(md: &mut MarkdownIt) {
  md.block.add_rule::<SectionScanner>();
}

#[derive(Debug)]
struct SectionStart {
  hash: String,
}

impl NodeValue for SectionStart {
  fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
    let (subject, _) = crate::git::message(&self.hash); // kinda wasteful

    fmt.cr();
    fmt.open("section", &node.attrs);
    fmt.cr();
    fmt.open("header", &[]);
    fmt.cr();
    fmt.text_raw(&format!(
      "<cite>{subject}<span class='commit_hash'><a href='{commit}'>{short_commit}</a></span></cite>",
      commit = self.hash,
      short_commit = &self.hash[0..7]
    ));
    fmt.cr();
    fmt.close("header");
    fmt.cr();
  }
}

#[derive(Debug)]
struct SectionEnd;

impl NodeValue for SectionEnd {
  fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
    fmt.cr();
    fmt.contents(&node.children);
    fmt.cr();
    fmt.close("section");
    fmt.cr();
  }
}
