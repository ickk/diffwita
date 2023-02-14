use crate::markers::Marker;
use markdown_it::{
  parser::block::{BlockRule, BlockState},
  MarkdownIt, Node, NodeValue, Renderer,
};

pub struct PatchsetScanner;

impl BlockRule for PatchsetScanner {
  fn run(state: &mut BlockState) -> Option<(Node, usize)> {
    let line = state.get_line(state.line).trim();

    if let Some(captures) = Marker::patchset_regex().captures(line) {
      let section_hash = captures.name("section_hash").unwrap().as_str().into();
      Some((Node::new(Patchset { hash: section_hash }), 1))
    } else {
      None
    }
  }
}

pub fn add_patchset_rules(md: &mut MarkdownIt) {
  md.block.add_rule::<PatchsetScanner>();
}

#[derive(Debug)]
struct Patchset {
  hash: String,
}

impl NodeValue for Patchset {
  fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
    let patchset = crate::git::patchset(&self.hash); // kinda wasteful

    let mut attrs = node.attrs.clone();
    attrs.push(("class", "patchset".to_owned()));

    fmt.cr();
    fmt.open("div", &attrs);
    fmt.cr();
    fmt.open("pre", &[]);
    fmt.open("code", &[]);
    fmt.text(&render_patchset(&patchset));
    fmt.close("code");
    fmt.close("pre");
    fmt.cr();
    fmt.close("div");
    fmt.cr();
  }
}

pub fn render_patchset(patchset: &unidiff::PatchSet) -> String {
  patchset.to_string()
}
