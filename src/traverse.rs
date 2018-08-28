use graphql_parser::query::*;
use visitor::*;

pub fn traverse(document: &Document, visitor: Box<Visitor>) {
  let mut traversal = Traversal { visitor };
  traversal.handle_document(&document);
}

struct Traversal {
  visitor: Box<Visitor>,
}

impl Traversal {
  fn handle_document(&mut self, document: &Document) {
    self.visitor.visit_document(document);

    for definition in &document.definitions {
      match definition {
        Definition::Operation(operation) => self.handle_operation_definition(&operation),
        Definition::Fragment(fragment) => self.handle_fragment_definition(&fragment),
      }
    }
  }

  fn handle_operation_definition(&mut self, operation_definition: &OperationDefinition) {
    self
      .visitor
      .visit_operation_definition(operation_definition);

    match operation_definition {
      OperationDefinition::SelectionSet(selection_set) => self.handle_selection_set(&selection_set),
      OperationDefinition::Query(query) => self.handle_query(&query),
      _ => (),
    }
  }

  fn handle_query(&mut self, query: &Query) {
    self.visitor.visit_query(&query);

    if let Some(name) = &query.name {
      self.handle_name(&name);
    }

    for variable_definition in &query.variable_definitions {
      self.handle_variable_definition(&variable_definition);
    }

    for directive in &query.directives {
      self.handle_directive(&directive);
    }

    self.handle_selection_set(&query.selection_set);
  }

  fn handle_variable_definition(&mut self, variable_definition: &VariableDefinition) {
    self.visitor.visit_variable_definition(&variable_definition);
    self.handle_name(&variable_definition.name);
    self.handle_type(&variable_definition.var_type);
    if let Some(default_value) = &variable_definition.default_value {
      self.handle_value(&default_value);
    }
  }

  fn handle_selection_set(&mut self, selection_set: &SelectionSet) {
    self.visitor.visit_selection_set(&selection_set);
    for selection in &selection_set.items {
      self.handle_selection(&selection);
    }
  }

  fn handle_selection(&mut self, selection: &Selection) {
    self.visitor.visit_selection(&selection);
    match &selection {
      Selection::Field(field) => self.handle_field(&field),
      Selection::FragmentSpread(fragment_spread) => self.handle_fragment_spread(&fragment_spread),
      Selection::InlineFragment(inline_fragment) => self.handle_inline_fragment(&inline_fragment),
    }
  }

  fn handle_field(&mut self, field: &Field) {
    self.visitor.visit_field(&field);

    if let Some(alias) = &field.alias {
      self.handle_name(&alias);
    }

    self.handle_name(&field.name);

    for argument in &field.arguments {
      self.handle_name(&argument.0);
      self.handle_value(&argument.1);
    }

    for directive in &field.directives {
      self.handle_directive(&directive);
    }

    self.handle_selection_set(&field.selection_set);
  }

  fn handle_fragment_definition(&mut self, fragment_definition: &FragmentDefinition) {
    self.visitor.visit_fragment_definition(&fragment_definition);
    self.handle_name(&fragment_definition.name);
    self.handle_type_condition(&fragment_definition.type_condition);
    for directive in &fragment_definition.directives {
      self.handle_directive(&directive);
    }

    self.handle_selection_set(&fragment_definition.selection_set);
  }

  fn handle_inline_fragment(&mut self, inline_fragment: &InlineFragment) {
    self.visitor.visit_inline_fragment(&inline_fragment);
    if let Some(type_condition) = &inline_fragment.type_condition {
      self.handle_type_condition(&type_condition);
    }

    for directive in &inline_fragment.directives {
      self.handle_directive(&directive);
    }
  }

  fn handle_fragment_spread(&mut self, fragment_spread: &FragmentSpread) {
    self.visitor.visit_fragment_spread(&fragment_spread);
    self.handle_name(&fragment_spread.fragment_name);
    for directive in &fragment_spread.directives {
      self.handle_directive(&directive);
    }
  }

  fn handle_directive(&mut self, directive: &Directive) {
    self.visitor.visit_directive(&directive);

    for argument in &directive.arguments {
      self.handle_name(&argument.0);
      self.handle_value(&argument.1);
    }
  }

  fn handle_type_condition(&mut self, type_condition: &TypeCondition) {
    self.visitor.visit_type_condition(&type_condition);

    match type_condition {
      TypeCondition::On(name) => self.handle_name(&name),
    }
  }

  fn handle_type(&mut self, type1: &Type) {
    self.visitor.visit_type(&type1);

    match &type1 {
      Type::NamedType(type2) => self.handle_name(&type2),
      Type::ListType(type2) => self.handle_type(&*type2),
      Type::NonNullType(type2) => self.handle_type(&*type2),
    }
  }

  fn handle_name(&mut self, name: &Name) {
    self.visitor.visit_name(&name);
  }

  fn handle_value(&mut self, value: &Value) {
    self.visitor.visit_value(&value);
  }
}
