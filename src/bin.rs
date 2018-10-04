extern crate graphql_id;

use graphql_id::*;

fn main() {
  let operation = select_operation(
    &"query A { foo { ...bar } ...bar } query B { ...foo } fragment foo on B { bar } fragment bar on A { foo }",
    &"A",
  );

  print!("{}", operation.unwrap());
}
