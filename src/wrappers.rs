use graphql_parser::query::*;
use std::cmp::{Ord, Ordering};

#[derive(Debug, Clone)]
pub struct FragmentDefinitionContainer<'a>(pub &'a FragmentDefinition);

impl<'a> Eq for FragmentDefinitionContainer<'a> {}
impl<'a> PartialEq for FragmentDefinitionContainer<'a> {
    fn eq(&self, other: &FragmentDefinitionContainer) -> bool {
        self.0.name.eq(&other.0.name)
    }
    fn ne(&self, other: &FragmentDefinitionContainer) -> bool {
        self.0.name.ne(&other.0.name)
    }
}
impl<'a> PartialOrd for FragmentDefinitionContainer<'a> {
    fn partial_cmp(&self, other: &FragmentDefinitionContainer) -> Option<Ordering> {
        self.0.name.partial_cmp(&other.0.name)
    }
}
impl<'a> Ord for FragmentDefinitionContainer<'a> {
    fn cmp(&self, other: &FragmentDefinitionContainer) -> Ordering {
        self.0.name.cmp(&other.0.name)
    }
}
