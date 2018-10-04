extern crate graphql_id;
extern crate graphql_parser;

use graphql_id::error::*;
use graphql_id::traverse::*;
use graphql_id::visitor::*;
use graphql_parser::query::*;

#[test]
fn name_visitor() {
    struct NameVisitor;

    impl Visitor for NameVisitor {
        fn visit_query_enter(&mut self, query: &Query) -> Result<(), GraphQLError> {
            assert_eq!(query.name.as_ref().unwrap(), "Foo");
            Ok(())
        }
    }

    let document = parse_query("query Foo { foo }").unwrap();
    let mut visitor = NameVisitor {};

    let mut traversal = Traversal {
        visitor: &mut visitor,
    };

    assert_eq!(traversal.handle_document(&document).is_ok(), true);
}
