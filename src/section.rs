use crate::markers::Marker;
use markdown_it::{
  parser::block::{BlockRule, BlockState},
  MarkdownIt, Node, NodeValue, Renderer,
};

pub struct SectionScanner;

impl BlockRule for SectionScanner {
  fn run(state: &mut BlockState) -> Option<(Node, usize)> {
    let line = state.get_line(state.line).trim();

    if let Some(captures) = Marker::section_begin_regex().captures(line) {
      let section_hash = captures.name("section_hash").unwrap().as_str().into();
      Some((Node::new(SectionStart { hash: section_hash }), 1))
    } else if Marker::section_end_regex().is_match(line) {
      Some((Node::new(SectionEnd), 1))
    } else {
      None
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
