use crate::markers::Marker;
use markdown_it::{
  parser::block::{BlockRule, BlockState},
  MarkdownIt, Node, NodeValue, Renderer,
};
use unidiff::PatchedFile;

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
    render_patchset(&patchset, fmt);
    fmt.cr();
    fmt.close("div");
    fmt.cr();
  }
}

#[derive(Debug)]
enum FileType {
  New,
  Deleted,
  Moved,
  Changed,
}

pub fn render_patchset(patchset: &unidiff::PatchSet, fmt: &mut dyn Renderer) {
  for patched_file in patchset.files() {
    let mut attrs = vec![("class", "file".to_owned())];
    let filetype = determine_filetype(patched_file);
    match &filetype {
      FileType::New => attrs.push(("class", "new".to_owned())),
      FileType::Deleted => attrs.push(("class", "deleted".to_owned())),
      FileType::Changed => attrs.push(("class", "changed".to_owned())),
      FileType::Moved => attrs.push(("class", "moved".to_string())),
    }
    fmt.cr();
    match &filetype {
      FileType::Moved => fmt.text_raw(&format!(
        r#"<span class="filename moved"><code>{src}</code> -> <code>{dst}</code></span>"#,
        src = &patched_file.source_file[2..],
        dst = &patched_file.target_file[2..]
      )),
      FileType::Deleted => {
        fmt.text_raw(&format!(
          r#"<span class="filename deleted"><code>{src}</code></span>"#,
          src = &patched_file.source_file[2..]
        ));
        continue;
      },
      FileType::New => fmt.text_raw(&format!(
        r#"<span class="filename new"><code>{dst}</code></span>"#,
        dst = &patched_file.target_file[2..]
      )),
      FileType::Changed => fmt.text_raw(&format!(
        r#"<span class="filename changed"><code>{dst}</code></span>"#,
        dst = &patched_file.target_file[2..]
      )),
    }
    fmt.open("div", &attrs);

    let mut ellipsis = None;
    for hunk in patched_file.hunks() {
      fmt.cr();
      if ellipsis.is_some() {
        fmt.text_raw("<div class=ellipsis></div>");
      }
      fmt.text_raw(&format!(r#"<div class="hunk">"#));
      fmt.text_raw(&format!(r"<p><pre><code>"));
      for line in hunk.lines() {
        let attrs = if line.is_added() {
          vec![
            ("class", "added".to_string()),
            ("data-lineno", "+".to_owned()),
          ]
        } else if line.is_removed() {
          vec![
            ("class", "removed".to_string()),
            (
              "data-lineno",
              line.source_line_no.unwrap_or_default().to_string(),
            ),
          ]
        } else {
          vec![
            ("class", "context".to_string()),
            (
              "data-lineno",
              line.source_line_no.unwrap_or_default().to_string(),
            ),
          ]
        };
        fmt.open("span", &attrs);
        let text = &line.value;
        if text.len() > 0 {
          fmt.text(text);
        } else {
          fmt.text(" ")
        }

        fmt.close("span");
        fmt.cr();
      }
      fmt.cr();
      fmt.text_raw(&format!(r#"</pre></code></p>"#));
      fmt.close("div");
      ellipsis = Some(());
    }
    fmt.cr();
    fmt.close("div");
    fmt.cr();
  }
}

fn determine_filetype(patched_file: &PatchedFile) -> FileType {
  if patched_file.source_file == "/dev/null" {
    FileType::New
  } else if patched_file.target_file == "/dev/null" {
    FileType::Deleted
  } else if patched_file.source_file[1..] != patched_file.target_file[1..] {
    FileType::Moved
  } else {
    FileType::Changed
  }
}
