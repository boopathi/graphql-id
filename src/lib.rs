extern crate graphql_parser;

mod traverse;
mod visitor;

use graphql_parser::query::*;
use traverse::*;
use visitor::Visitor;

#[derive(Debug, Clone, Copy)]
struct PrintVisitor;
impl Visitor for PrintVisitor {
    fn visit_document(&mut self, _document: &Document) {
        println!("visiting document node");
    }
    fn visit_name(&mut self, name: &Name) {
        let name = &name.clone();
        println!("{}", name);
    }
}

pub fn callme() {
    let document = parse_query("query Foo{foo}").unwrap();
    let visitor = Box::new(PrintVisitor {});

    traverse(&document, visitor);
}
