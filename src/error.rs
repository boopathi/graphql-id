use graphql_parser::query::*;
use std::fmt;

#[derive(Debug)]
pub enum GraphQLError {
  OperationNotFound,
  FragmentNotFound,
  InfiniteFragmentRecursionError(String),
  Parse(ParseError),
  MultipleOperation,
  AnonymousOperation,
}

impl fmt::Display for GraphQLError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      GraphQLError::Parse(ref e) => e.fmt(f),
      GraphQLError::OperationNotFound => {
        write!(f, "The input operation is not found in the document")
      }
      GraphQLError::FragmentNotFound => {
        write!(f, "The query does not contain the fragment which is used")
      }
      GraphQLError::InfiniteFragmentRecursionError(ref fragment_name) => {
        write!(f, "Infinte Fragment Recursion detected {}", fragment_name)
      }
      GraphQLError::MultipleOperation => write!(
        f,
        "Multiple Operations found in the query. Operation Name is required"
      ),
      GraphQLError::AnonymousOperation => write!(
        f,
        "Cannot generate id for anonymous query. Use an operation name"
      ),
    }
  }
}

impl From<ParseError> for GraphQLError {
  fn from(err: ParseError) -> GraphQLError {
    GraphQLError::Parse(err)
  }
}
