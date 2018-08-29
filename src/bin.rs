extern crate graphql_select;

use graphql_select::*;

fn main() {
  let operation = select_operation(
    &"query A { foo } query B { ...foo } fragment foo on B { bar }",
    &"B",
  );

  println!("{}", operation.unwrap());
}
