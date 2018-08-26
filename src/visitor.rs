use graphql_parser::query::*;

#[allow(unused_variables)]
pub trait Visitor {
  fn visit_definition(&mut self, definition: &Definition) -> () {}
  fn visit_directive(&mut self, directive: &Directive) -> () {}
  fn visit_document(&mut self, document: &Document) -> () {}
  fn visit_field(&mut self, field: &Field) -> () {}
  fn visit_fragment_definition(&mut self, fragment_definition: &FragmentDefinition) -> () {}
  fn visit_fragment_spread(&mut self, fragment_spread: &FragmentSpread) -> () {}
  fn visit_inline_fragment(&mut self, inline_fragment: &InlineFragment) -> () {}
  fn visit_mutation(&mut self, mutation: &Mutation) -> () {}
  fn visit_name(&mut self, name: &Name) -> () {}
  fn visit_number(&mut self, number: &Number) -> () {}
  fn visit_operation_definition(&mut self, operation_defintion: &OperationDefinition) -> () {}
  fn visit_query(&mut self, query: &Query) -> () {}
  fn visit_selection_set(&mut self, selection_set: &SelectionSet) -> () {}
  fn visit_selection(&mut self, selection: &Selection) -> () {}
  fn visit_type_condition(&mut self, type_condition: &TypeCondition) -> () {}
  fn visit_type(&mut self, type1: &Type) -> () {}
  fn visit_value(&mut self, value: &Value) -> () {}
  fn visit_variable_definition(&mut self, variable_definition: &VariableDefinition) -> () {}
}
