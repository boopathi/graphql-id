extern crate clap;
extern crate graphql_id;

use clap::{App, Arg};
use graphql_id::*;
use std::fs::File;
use std::io::prelude::Read;

fn main() {
  let matches = App::new("graphql-id")
    .version("0.1.0")
    .about("generate graphql id")
    .arg(
      Arg::with_name("FILE")
        .help("file to select")
        .required(true)
        .index(1),
    ).arg(
      Arg::with_name("OPERATION_NAME")
        .help("operation name")
        .required(false)
        .index(2),
    ).get_matches();

  let file = matches.value_of("FILE").unwrap();

  let mut f = File::open(file).expect(format!("File NOT FOUND: {}", file).as_str());
  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect(format!("Error reading file: {}", file).as_str());

  let id = match matches.value_of("OPERATION_NAME") {
    Some(operation_name) => generate_operation_id(&contents.to_string(), operation_name),
    None => generate_default_operation_id(&contents.to_string()),
  };

  println!("{}", id.unwrap());
}
