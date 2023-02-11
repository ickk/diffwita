use diffguide::*;

pub fn main() {
    // println!("message(\"32cd65\"):\n{:}", git::message("32cd65"));
    // println!("log(\"32cd65\"):\n{:#?}", git::log("32cd65"));
    println!("{:#?}", git::patchset("32cd65"));
  }
