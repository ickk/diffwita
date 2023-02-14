use crate::{annotations::Annotation, git::Section};
use markdown_it::{
  parser::block::{BlockRule, BlockState},
  plugins::cmark,
  MarkdownIt, Node, NodeValue, Renderer,
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::cell::RefCell;
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
      patchsets.push((section.commit, section.patchset));
      text.push_str(&section.text);
      (text, patchsets)
    },
  );
  (text, PatchSetsCollection(patchsets))
}

#[derive(Debug)]
pub struct PatchSetsCollection(pub(crate) Vec<(String, PatchSet)>);

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

// this is way too complicated. Should just annotate with the commit hash
// directly; then the render function can call git::patchset directly rather
// than keeping everything in memory and using this ugly global state hack.

pub struct SectionScanner;

use crate::annotations::{MARKER, SECTION_BEGIN, SECTION_END, SEP};
static SECTION_BEGIN_RE: Lazy<Regex> = Lazy::new(|| {
  Regex::new(&format!(
    r"^{MARKER}{SEP}{SECTION_BEGIN}{SEP}(?P<section_id>\d+){SEP}{MARKER}$"
  ))
  .unwrap()
});
static SECTION_END_RE: Lazy<Regex> = Lazy::new(|| {
  Regex::new(&format!(
    r"^{MARKER}{SEP}{SECTION_END}{SEP}(?P<section_id>\d+){SEP}{MARKER}$"
  ))
  .unwrap()
});

impl BlockRule for SectionScanner {
  fn run(state: &mut BlockState) -> Option<(Node, usize)> {
    let line = state.get_line(state.line).trim();

    if let Some(captures) = SECTION_BEGIN_RE.captures(line) {
      let section_id = captures
        .name("section_id")
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();
      Some((Node::new(SectionStart(section_id)), 1))
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
struct SectionStart(usize);

thread_local!(pub static SECTION_INFO: RefCell<Option<crate::parse::PatchSetsCollection>> = RefCell::new(None));

impl NodeValue for SectionStart {
  fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
    // get the data assosciated with this section
    let commit = SECTION_INFO.with(|cell| {
      let collection = cell.borrow();
      collection
        .as_ref()
        .expect("SECTION_INFO was not populated")
        .0[self.0] // find the `(commit, patchset)` pair at the section index
        .0 // we only care about the commit
        .to_owned()
    });

    fmt.cr();
    fmt.open("section", &node.attrs);
    fmt.cr();
    fmt.open("header", &[]);
    fmt.cr();
    fmt.text_raw(&format!(
      "<cite>{subject}<span class='commit_hash'><a href='{commit}'>{short_commit}</a></span></cite>",
      subject = " subject line .. ",
      commit = commit.to_owned(),
      short_commit = &commit[commit.len() - 7..]
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
