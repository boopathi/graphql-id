extern crate graphql_id;

use graphql_id::error::*;
use graphql_id::*;

#[test]
fn id_generation_selection_set() {
    let query = "
        {
            foo
            bar
        }
    ";

    let id = generate_default_operation_id(&query);
    match id {
        Err(e) => match e {
            GraphQLError::AnonymousOperation => assert!(true),
            _ => assert!(false),
        },
        _ => assert!(false),
    };
}

#[test]
fn id_generation_anonymous_query() {
    let query = "
        query {
            foo
            bar
        }
    ";

    let id = generate_default_operation_id(&query);
    match id {
        Err(e) => match e {
            GraphQLError::AnonymousOperation => assert!(true),
            _ => assert!(false),
        },
        _ => assert!(false),
    };
}

#[test]
fn default_id_generation_multiple_queries() {
    let query = "
        query A {
            foo
            bar
        }
        query B {
            foo
            bar
        }
    ";

    let id = generate_default_operation_id(&query);
    match id {
        Err(e) => match e {
            GraphQLError::MultipleOperation => assert!(true),
            _ => assert!(false),
        },
        _ => assert!(false),
    };
}

#[test]
fn default_id_generation() {
    let query = "
        query A {
            foo
        }
    ";

    let id = generate_default_operation_id(&query);
    assert_eq!(id.is_ok(), true);
}
