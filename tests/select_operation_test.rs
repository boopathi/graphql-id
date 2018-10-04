extern crate graphql_id;
extern crate graphql_parser;

use graphql_id::select_operation;
use graphql_parser::parse_query;

enum QueryStr {
    Actual(String),
    Expected(&'static str),
}

fn normalize(query: QueryStr) -> String {
    let query = match query {
        QueryStr::Actual(query) => query.to_string(),
        QueryStr::Expected(query) => query.to_string(),
    };
    format!("{}", parse_query(query.as_ref()).unwrap())
}

#[test]
fn single_operation_should_be_no_op() {
    let query = "
        query A {
          foo
        }
    ";

    assert_eq!(
        normalize(QueryStr::Actual(select_operation(&query, &"A").unwrap())),
        normalize(QueryStr::Expected(query))
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
        normalize(QueryStr::Actual(select_operation(&query, &"A").unwrap())),
        normalize(QueryStr::Expected(
            "
                query A {
                    foo
                }
            "
        ))
    );
}
