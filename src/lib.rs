extern crate crypto;
extern crate graphql_parser;
extern crate itertools;

pub mod error;
pub mod traverse;
pub mod visitor;

mod wrappers;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use error::*;
use graphql_parser::query::*;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
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
            .filter_map(|definition| filter_fragment_defintion(definition))
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

/// Generate an operation id for a query with an operation_name
///
/// # Examples
///
/// ```
///
/// ```
///
pub fn generate_operation_id(query: &str, operation_name: &str) -> Result<String, GraphQLError> {
    let mut hasher = Sha256::new();
    let operation = select_operation(query, operation_name)?;
    hasher.input_str(operation.as_str());

    Ok(hasher.result_str())
}

pub fn generate_default_operation_id(query: &str) -> Result<String, GraphQLError> {
    let operation_name = get_default_operation_name(query)?;

    generate_operation_id(query, operation_name.as_str())
}

pub fn get_default_operation_name(query: &str) -> Result<String, GraphQLError> {
    let document = parse_query(query)?;

    document
        .definitions
        .iter()
        .filter_map(|definition| filter_operation_definition(definition))
        .fold_while(
            Err(GraphQLError::OperationNotFound),
            |result, operation_definition| match operation_definition {
                OperationDefinition::Query(query) => {
                    if result.is_ok() {
                        Done(Err(GraphQLError::MultipleOperation))
                    } else {
                        Continue(query.clone().name.ok_or(GraphQLError::AnonymousOperation))
                    }
                }
                OperationDefinition::Mutation(mutation) => {
                    if result.is_ok() {
                        Done(Err(GraphQLError::MultipleOperation))
                    } else {
                        Continue(
                            mutation
                                .clone()
                                .name
                                .ok_or(GraphQLError::AnonymousOperation),
                        )
                    }
                }
                OperationDefinition::SelectionSet(_) => Done(Err(GraphQLError::AnonymousOperation)),
                OperationDefinition::Subscription(_) => Done(Err(GraphQLError::AnonymousOperation)),
            },
        )
        .into_inner()
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
        .filter_map(|definition| filter_operation_definition(definition))
        .find(|operation_definition| match operation_definition {
            OperationDefinition::Query(query) => is_operation_name(&query.name, &operation_name),
            OperationDefinition::Mutation(mutation) => {
                is_operation_name(&mutation.name, &operation_name)
            }
            _ => false,
        })
}

fn filter_operation_definition(definition: &Definition) -> Option<&OperationDefinition> {
    match definition {
        Definition::Operation(operation_definition) => Some(&operation_definition),
        _ => None,
    }
}

fn filter_fragment_defintion(definition: &Definition) -> Option<&FragmentDefinition> {
    match definition {
        Definition::Fragment(fragment_definition) => Some(&fragment_definition),
        _ => None,
    }
}

fn is_operation_name(name: &Option<String>, operation_name: &str) -> bool {
    name.as_ref().map_or(false, |name| name == operation_name)
}
