extern crate graphql_parser;
extern crate graphql_select;

use graphql_parser::parse_query;
use graphql_select::select_operation;

enum QueryStr {
  Actual(String),
  Expected(&'static str),
}

fn normalize(query: QueryStr) -> String {
  let query = match query {
    QueryStr::Actual(query) => query.to_string(),
    QueryStr::Expected(query) => query,
  };
  format!("{}", parse_query(query).unwrap())
}

#[test]
fn single_operation_should_be_no_op() {
  let query = "
    query A {
      foo
    }
  ";

  assert_eq!(
    normalize(select_operation(&query, &"A").unwrap()),
    normalize(query)
  );
}

#[test]
fn select_from_multiple_queries() {
  let query = "
    query A {
      foo
    }
    query B {
      bar
    }
  ";

  assert_eq!(
    normalize(select_operation(&query, &"A").unwrap()),
    normalize(
      "
        query A {
          foo
        }
      "
    )
  );
}
