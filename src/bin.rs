extern crate graphql_select;

use graphql_select::*;

fn main() {
  let operation = select_operation(
    &"query A { foo { ...bar } ...bar } query B { ...foo } fragment foo on B { bar } fragment bar on A { foo }",
    &"A",
  );

  print!("{}", operation.unwrap());
}
