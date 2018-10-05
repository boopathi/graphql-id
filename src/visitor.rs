use error::*;
use graphql_parser::query::*;

#[allow(unused_variables)]
pub trait Visitor {
    fn visit_definition_enter(&mut self, definition: &Definition) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_definition_exit(&mut self, definition: &Definition) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_directive_enter(&mut self, directive: &Directive) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_directive_exit(&mut self, directive: &Directive) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_document_enter(&mut self, document: &Document) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_document_exit(&mut self, document: &Document) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_field_enter(&mut self, field: &Field) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_field_exit(&mut self, field: &Field) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_fragment_definition_enter(
        &mut self,
        fragment_definition: &FragmentDefinition,
    ) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_fragment_definition_exit(
        &mut self,
        fragment_definition: &FragmentDefinition,
    ) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_fragment_spread_enter(
        &mut self,
        fragment_spread: &FragmentSpread,
    ) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_fragment_spread_exit(
        &mut self,
        fragment_spread: &FragmentSpread,
    ) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_inline_fragment_enter(
        &mut self,
        inline_fragment: &InlineFragment,
    ) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_inline_fragment_exit(
        &mut self,
        inline_fragment: &InlineFragment,
    ) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_mutation_enter(&mut self, mutation: &Mutation) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_mutation_exit(&mut self, mutation: &Mutation) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_name_enter(&mut self, name: &Name) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_name_exit(&mut self, name: &Name) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_number_enter(&mut self, number: &Number) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_number_exit(&mut self, number: &Number) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_operation_definition_enter(
        &mut self,
        operation_defintion: &OperationDefinition,
    ) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_operation_definition_exit(
        &mut self,
        operation_defintion: &OperationDefinition,
    ) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_query_enter(&mut self, query: &Query) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_query_exit(&mut self, query: &Query) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_selection_set_enter(
        &mut self,
        selection_set: &SelectionSet,
    ) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_selection_set_exit(
        &mut self,
        selection_set: &SelectionSet,
    ) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_selection_enter(&mut self, selection: &Selection) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_selection_exit(&mut self, selection: &Selection) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_type_condition_enter(
        &mut self,
        type_condition: &TypeCondition,
    ) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_type_condition_exit(
        &mut self,
        type_condition: &TypeCondition,
    ) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_type_enter(&mut self, type1: &Type) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_type_exit(&mut self, type1: &Type) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_value_enter(&mut self, value: &Value) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_value_exit(&mut self, value: &Value) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_variable_definition_enter(
        &mut self,
        variable_definition: &VariableDefinition,
    ) -> Result<(), GraphQLError> {
        Ok(())
    }
    fn visit_variable_definition_exit(
        &mut self,
        variable_definition: &VariableDefinition,
    ) -> Result<(), GraphQLError> {
        Ok(())
    }
}
