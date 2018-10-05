use graphql_parser::query::*;
use std::fmt;

/// # GraphQLError
#[derive(Debug)]
pub enum GraphQLError {
    /// The target operation is not found in the Query
    OperationNotFound,

    /// A fragment used in the target operation is not found
    FragmentNotFound,

    /// A fragment contains itself or contains other fragments that cause a loop
    InfiniteFragmentRecursionError(String),

    /// GraphQL Parse error
    Parse(ParseError),

    /// A default operation cannot be chosen as the input contains multiple operations
    MultipleOperation,

    /// An operation always requires an operation name (not a limitation in GraphQL. Just a limitation in this library)
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
