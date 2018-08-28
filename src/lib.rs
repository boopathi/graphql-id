extern crate graphql_parser;

pub mod traverse;
pub mod visitor;

use graphql_parser::query::*;
use traverse::*;
use visitor::Visitor;

struct PrintVisitor;
impl Visitor for PrintVisitor {
    fn visit_query(&mut self, query: &Query) {
        let name = &query.name;
        let name = name.as_ref();
        println!("{}", name.unwrap());
    }
}

pub fn callme() {
    let document = parse_query("query Foo{foo}").unwrap();
    let visitor = Box::new(PrintVisitor {});

    traverse(&document, visitor);
}
