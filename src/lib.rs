extern crate graphql_parser;

pub mod ast;
pub mod error;
pub mod traverse;
pub mod visitor;

mod wrappers;

use error::*;
use graphql_parser::query::*;
use std::collections::BTreeSet;
use std::result::Result::*;
use traverse::*;
use visitor::Visitor;
use wrappers::FragmentDefinitionContainer;

struct FragmentSpreadVisitor<'a> {
    /// the document reference to get to fragment node from fragment spread
    document: &'a Document,

    /// used fragments - our result that we will use to select the fragments
    used_fragments: Vec<&'a FragmentDefinition>,

    /// the visited fragments during traversal - so as to avoid infinite
    /// recursion of fragments
    visited_fragments: Vec<&'a FragmentDefinition>,
}

impl<'a> Visitor for FragmentSpreadVisitor<'a> {
    fn visit_fragment_spread_enter(
        &mut self,
        fragment_spread: &FragmentSpread,
    ) -> Result<(), GraphQLError> {
        let fragment = self
            .document
            .definitions
            .iter()
            .filter_map(|definition| select_fragment_defintion(definition))
            .find(|fragment_definition| fragment_definition.name == fragment_spread.fragment_name)
            .ok_or(GraphQLError::FragmentNotFound)?;

        let found = &self
            .visited_fragments
            .iter()
            .find(|visited| &visited.name == &fragment.name);

        if let Some(fragment) = found {
            return Err(GraphQLError::InfiniteFragmentRecursionError(
                fragment.name.clone(),
            ));
        }

        let mut used_fragments: BTreeSet<_> = self
            .used_fragments
            .iter()
            .map(|fragment| FragmentDefinitionContainer(fragment))
            .collect();
        used_fragments.insert(FragmentDefinitionContainer(fragment));

        let used_fragments = used_fragments
            .iter()
            .map(|container| container.0)
            .collect::<Vec<_>>();

        let mut sub_visitor = FragmentSpreadVisitor {
            document: &self.document,
            visited_fragments: [&self.visited_fragments[..], &[fragment]].concat(),
            used_fragments: used_fragments.clone(),
        };

        self.used_fragments = used_fragments;

        let mut traversal = Traversal {
            visitor: &mut sub_visitor,
        };

        traversal.handle_fragment_definition(&fragment)?;

        Ok(())
    }
}

pub fn select_operation(query: &str, operation_name: &str) -> Result<String, GraphQLError> {
    let document = parse_query(query)?;
    let operation = select_operation_definition(&document, &operation_name)
        .ok_or(GraphQLError::OperationNotFound)?;

    let mut visitor = FragmentSpreadVisitor {
        document: &document,
        used_fragments: vec![],
        visited_fragments: vec![],
    };

    // this block is required so the mutable borrow of visitor ends here
    {
        let mut traversal = Traversal {
            visitor: &mut visitor,
        };
        traversal.handle_operation_definition(&operation)?;
    }

    let fragments = visitor
        .used_fragments
        .iter()
        .map(|fragment| format!("{}", fragment))
        .collect::<Vec<String>>()
        .join("\n");

    Ok(format!("{}{}", operation, fragments))
}

fn select_operation_definition<'a>(
    document: &'a Document,
    operation_name: &'a str,
) -> Option<&'a OperationDefinition> {
    document
        .definitions
        .iter()
        .filter_map(|definition| choose_operation_definition(definition))
        .find(|operation_definition| match operation_definition {
            OperationDefinition::Query(query) => is_operation_name(&query.name, &operation_name),
            OperationDefinition::Mutation(mutation) => {
                is_operation_name(&mutation.name, &operation_name)
            }
            _ => false,
        })
}

fn choose_operation_definition(definition: &Definition) -> Option<&OperationDefinition> {
    match definition {
        Definition::Operation(operation_definition) => Some(&operation_definition),
        _ => None,
    }
}

fn select_fragment_defintion(definition: &Definition) -> Option<&FragmentDefinition> {
    match definition {
        Definition::Fragment(fragment_definition) => Some(&fragment_definition),
        _ => None,
    }
}

fn is_operation_name(name: &Option<String>, operation_name: &str) -> bool {
    name.as_ref().map_or(false, |name| name == operation_name)
}
