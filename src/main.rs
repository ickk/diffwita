use diffguide::*;

pub fn main() {
  git::log(&git::head()).iter().for_each(|commit_meta| {
    println!("{}", git::message(&commit_meta.commit));
    println!("{}", git::patchset(&commit_meta.commit));
  });
}
