use diffwita::*;
use once_cell::sync::Lazy;
use tera::{Context, Tera};

pub fn main() {
  git::configure();

  let tree = git::head();
  let ast = parse::parse(tree);
  let html = ast.render();

  // println!("{}", html);

  template(&html);
}

static TEMPLATES: Lazy<Tera> = Lazy::new(|| {
  let mut tera = match Tera::new("templates/*.tera") {
    Ok(t) => t,
    Err(e) => {
      eprintln!("Parsing error: {e}");
      ::std::process::exit(1);
    },
  };
  tera.autoescape_on(vec![]);
  tera
});

fn template(document: &str) {
  let mut context = Context::new();
  context.insert("document", document);

  let tera = &TEMPLATES;
  let html = tera
    .render("index.tera", &context)
    .expect("tera failed to render index.html");

  println!("{html}");
}
