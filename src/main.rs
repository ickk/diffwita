use diffguide::*;

pub fn main() {
  git::configure();

  let tree = git::head();
  let log = git::log(&tree);
  let sections = log.sections();
  let annotated_sections = parse::annotate(sections);
  let text = parse::concat(annotated_sections);

  let ast = parse::parse(text);
  // eprintln!("{:#?}", ast);

  let html = ast.render();
  println!("{}", html);
}
