use diffguide::*;

pub fn main() {
  git::configure();

  let tree = git::head();
  let log = git::log(&tree);
  let sections = log.sections();
  let annotated_sections = parse::annotate(sections);
  let (text, patchsets) = parse::concat(annotated_sections);

  parse::SECTION_INFO.with(|cell| cell.replace(Some(patchsets)));

  let ast = parse::parse(text);
  // eprintln!("{:#?}", ast);

  let html = ast.render();
  println!("{}", html);
}
