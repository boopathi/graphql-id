extern crate graphql_id;
extern crate graphql_parser;

use graphql_id::*;
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

#[test]
fn select_with_single_fragment() {
    let query = "
        query A {
            ...foo
        }
        fragment foo on Foo {
            bar
        }
    ";

    assert_eq!(
        normalize(QueryStr::Actual(select_operation(&query, &"A").unwrap())),
        normalize(QueryStr::Expected(&query))
    )
}

#[test]
fn select_with_multiple_fragments() {
    let query = "
        query A {
            ...foo
        }
        query B {
            ...bar
        }
        fragment foo on Foo {
            foo
        }
        fragment bar on Bar {
            bar
        }
    ";

    assert_eq!(
        normalize(QueryStr::Actual(select_operation(&query, &"A").unwrap())),
        normalize(QueryStr::Expected(
            "
                query A {
                    ...foo
                }
                fragment foo on Foo {
                    foo
                }
            "
        ))
    )
}
