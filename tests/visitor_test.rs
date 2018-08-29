extern crate graphql_parser;
extern crate graphql_select;

use graphql_parser::query::*;
use graphql_select::traverse::*;
use graphql_select::visitor::*;

#[test]
fn name_visitor() {
  struct NameVisitor;
  impl Visitor for NameVisitor {
    fn visit_query_enter<'a>(&mut self, query: &'a Query) {
      assert_eq!(query.name.as_ref().unwrap(), "Foo");
    }
  }

  let document = parse_query("query Foo { foo }").unwrap();
  let mut visitor = NameVisitor {};

  let mut traversal = Traversal {
    visitor: &mut visitor,
  };

  traversal.handle_document(&document);
}
