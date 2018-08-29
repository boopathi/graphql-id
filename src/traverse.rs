use error::*;
use graphql_parser::query::*;
use visitor::*;

// pub fn traverse_document(document: &Document, visitor: Box<Visitor>) -> Result<(), GraphQLError> {
//   let mut traversal = Traversal { visitor };
//   traversal.handle_document(&document);
// }

pub struct Traversal<'a> {
  pub visitor: &'a mut Visitor,
}

impl<'a> Traversal<'a> {
  pub fn handle_document(&mut self, document: &Document) -> Result<(), GraphQLError> {
    self.visitor.visit_document_enter(document)?;

    for definition in &document.definitions {
      match definition {
        Definition::Operation(operation) => self.handle_operation_definition(&operation),
        Definition::Fragment(fragment) => self.handle_fragment_definition(&fragment),
      }?;
    }

    self.visitor.visit_document_exit(document)
  }

  pub fn handle_operation_definition(
    &mut self,
    operation_definition: &OperationDefinition,
  ) -> Result<(), GraphQLError> {
    self
      .visitor
      .visit_operation_definition_enter(operation_definition)?;

    match operation_definition {
      OperationDefinition::SelectionSet(selection_set) => self.handle_selection_set(&selection_set),
      OperationDefinition::Query(query) => self.handle_query(&query),
      _ => Ok(()),
    }?;

    self
      .visitor
      .visit_operation_definition_exit(operation_definition)
  }

  pub fn handle_query(&mut self, query: &Query) -> Result<(), GraphQLError> {
    self.visitor.visit_query_enter(&query)?;
    if let Some(name) = &query.name {
      self.handle_name(&name)?;
    }
    for variable_definition in &query.variable_definitions {
      self.handle_variable_definition(&variable_definition)?;
    }
    for directive in &query.directives {
      self.handle_directive(&directive)?;
    }
    self.handle_selection_set(&query.selection_set)?;
    self.visitor.visit_query_exit(&query)
  }

  pub fn handle_variable_definition(
    &mut self,
    variable_definition: &VariableDefinition,
  ) -> Result<(), GraphQLError> {
    self
      .visitor
      .visit_variable_definition_enter(&variable_definition)?;
    self.handle_name(&variable_definition.name)?;
    self.handle_type(&variable_definition.var_type)?;
    if let Some(default_value) = &variable_definition.default_value {
      self.handle_value(&default_value)?;
    }
    self
      .visitor
      .visit_variable_definition_exit(&variable_definition)
  }

  pub fn handle_selection_set(&mut self, selection_set: &SelectionSet) -> Result<(), GraphQLError> {
    self.visitor.visit_selection_set_enter(&selection_set)?;
    for selection in &selection_set.items {
      self.handle_selection(&selection)?;
    }
    self.visitor.visit_selection_set_exit(&selection_set)
  }

  pub fn handle_selection(&mut self, selection: &Selection) -> Result<(), GraphQLError> {
    self.visitor.visit_selection_enter(&selection)?;
    match &selection {
      Selection::Field(field) => self.handle_field(&field),
      Selection::FragmentSpread(fragment_spread) => self.handle_fragment_spread(&fragment_spread),
      Selection::InlineFragment(inline_fragment) => self.handle_inline_fragment(&inline_fragment),
    }?;
    self.visitor.visit_selection_exit(&selection)
  }

  pub fn handle_field(&mut self, field: &Field) -> Result<(), GraphQLError> {
    self.visitor.visit_field_enter(&field)?;
    if let Some(alias) = &field.alias {
      self.handle_name(&alias)?;
    }
    self.handle_name(&field.name)?;
    for argument in &field.arguments {
      self.handle_name(&argument.0)?;
      self.handle_value(&argument.1)?;
    }
    for directive in &field.directives {
      self.handle_directive(&directive)?;
    }
    self.handle_selection_set(&field.selection_set)?;
    self.visitor.visit_field_exit(&field)
  }

  pub fn handle_fragment_definition(
    &mut self,
    fragment_definition: &FragmentDefinition,
  ) -> Result<(), GraphQLError> {
    self
      .visitor
      .visit_fragment_definition_enter(&fragment_definition)?;
    self.handle_name(&fragment_definition.name)?;
    self.handle_type_condition(&fragment_definition.type_condition)?;
    for directive in &fragment_definition.directives {
      self.handle_directive(&directive)?;
    }

    self.handle_selection_set(&fragment_definition.selection_set)?;
    self
      .visitor
      .visit_fragment_definition_exit(&fragment_definition)
  }

  pub fn handle_inline_fragment(
    &mut self,
    inline_fragment: &InlineFragment,
  ) -> Result<(), GraphQLError> {
    self.visitor.visit_inline_fragment_enter(&inline_fragment)?;
    if let Some(type_condition) = &inline_fragment.type_condition {
      self.handle_type_condition(&type_condition)?;
    }

    for directive in &inline_fragment.directives {
      self.handle_directive(&directive)?;
    }
    self.visitor.visit_inline_fragment_exit(&inline_fragment)
  }

  pub fn handle_fragment_spread(
    &mut self,
    fragment_spread: &FragmentSpread,
  ) -> Result<(), GraphQLError> {
    self.visitor.visit_fragment_spread_enter(&fragment_spread)?;
    self.handle_name(&fragment_spread.fragment_name)?;
    for directive in &fragment_spread.directives {
      self.handle_directive(&directive)?;
    }
    self.visitor.visit_fragment_spread_exit(&fragment_spread)
  }

  pub fn handle_directive(&mut self, directive: &Directive) -> Result<(), GraphQLError> {
    self.visitor.visit_directive_enter(&directive)?;

    for argument in &directive.arguments {
      self.handle_name(&argument.0)?;
      self.handle_value(&argument.1)?;
    }
    self.visitor.visit_directive_exit(&directive)
  }

  pub fn handle_type_condition(
    &mut self,
    type_condition: &TypeCondition,
  ) -> Result<(), GraphQLError> {
    self.visitor.visit_type_condition_enter(&type_condition)?;

    match type_condition {
      TypeCondition::On(name) => self.handle_name(&name),
    }?;
    self.visitor.visit_type_condition_exit(&type_condition)
  }

  pub fn handle_type(&mut self, type1: &Type) -> Result<(), GraphQLError> {
    self.visitor.visit_type_enter(&type1)?;

    match &type1 {
      Type::NamedType(type2) => self.handle_name(&type2),
      Type::ListType(type2) => self.handle_type(&*type2),
      Type::NonNullType(type2) => self.handle_type(&*type2),
    }?;
    self.visitor.visit_type_exit(&type1)
  }

  pub fn handle_name(&mut self, name: &Name) -> Result<(), GraphQLError> {
    self.visitor.visit_name_enter(&name)?;
    self.visitor.visit_name_exit(&name)
  }

  pub fn handle_value(&mut self, value: &Value) -> Result<(), GraphQLError> {
    self.visitor.visit_value_enter(&value)?;
    self.visitor.visit_value_exit(&value)
  }
}
