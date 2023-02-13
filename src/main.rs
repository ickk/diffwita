use diffguide::*;

pub fn main() {
  git::configure();

  let tree = git::head();
  let log = git::log(&tree);
  let sections = log.sections();
  let annotated_sections = parse::annotate(sections);
  let (text, _patchsets) = parse::concat(annotated_sections);

  let ast = parse::parse(text);
  // println!("{:#?}", ast);

  let html = ast.render();
  println!("{}", html);
}
