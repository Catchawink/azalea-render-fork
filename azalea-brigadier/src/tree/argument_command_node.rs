use std::fmt::{Display, Formatter};

use crate::{
    arguments::argument_type::ArgumentType,
    builder::required_argument_builder::RequiredArgumentBuilder,
    context::{
        command_context::CommandContext, command_context_builder::CommandContextBuilder,
        parsed_argument::ParsedArgument,
    },
    exceptions::command_syntax_exception::CommandSyntaxException,
    string_reader::StringReader,
    suggestion::{
        suggestion_provider::SuggestionProvider, suggestions::Suggestions,
        suggestions_builder::SuggestionsBuilder,
    },
};

use super::command_node::{BaseCommandNode, CommandNode};

const USAGE_ARGUMENT_OPEN: &str = "<";
const USAGE_ARGUMENT_CLOSE: &str = ">";

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct ArgumentCommandNode<'a, S, T> {
    name: String,
    type_: &'a T,
    custom_suggestions: &'a dyn SuggestionProvider<S, T>,
    // Since Rust doesn't have extending, we put the struct this is extending as the "base" field
    pub base: BaseCommandNode<'a, S, T>,
}

impl<S, T> ArgumentCommandNode<'_, S, T> {
    fn get_type(&self) -> &T {
        &self.type_
    }

    fn custom_suggestions(&self) -> &dyn SuggestionProvider<S, T> {
        &self.custom_suggestions
    }
}

impl<S, T> CommandNode<S, T> for ArgumentCommandNode<'_, S, T> {
    fn name(&self) -> &str {
        &self.name
    }

    fn parse(
        &self,
        reader: &mut StringReader,
        context_builder: CommandContextBuilder<S, T>,
    ) -> Result<(), CommandSyntaxException> {
        // final int start = reader.getCursor();
        // final T result = type.parse(reader);
        // final ParsedArgument<S, T> parsed = new ParsedArgument<>(start, reader.getCursor(), result);

        // contextBuilder.withArgument(name, parsed);
        // contextBuilder.withNode(this, parsed.getRange());

        let start = reader.get_cursor();
        let result = self.get_type().parse(reader)?;
        let parsed = ParsedArgument::new(start, reader.get_cursor(), result);

        context_builder.with_argument(&self.name, parsed);
        context_builder.with_node(self, parsed.get_range());

        Ok(())
    }

    fn list_suggestions(
        &self,
        context: CommandContext<S, T>,
        builder: &mut SuggestionsBuilder,
    ) -> Result<Suggestions, CommandSyntaxException> {
        if self.custom_suggestions.is_none() {
            self.get_type().list_suggestions(context, builder)
        } else {
            self.custom_suggestions.get_suggestions(context, builder)
        }
    }

    fn is_valid_input(&self, input: &str) -> bool {
        let reader = StringReader::new(input);
        let result = self.get_type().parse(reader);
        if result.is_ok() {
            return !reader.can_read() || reader.peek() == ' ';
        } else {
            return false;
        }
    }

    fn usage_text(&self) -> &str {
        USAGE_ARGUMENT_OPEN + self.name + USAGE_ARGUMENT_CLOSE
    }

    fn create_builder(&self) -> RequiredArgumentBuilder<S, T> {
        let builder = RequiredArgumentBuilder::argument(&self.name, &self.type_);
        builder.requires(self.base.get_requirement());
        builder.forward(
            self.base.get_redirect(),
            self.base.get_redirect_modifier(),
            self.base.is_fork(),
        );
        builder.suggests(self.custom_suggestions());
        if self.base.get_command() != None {
            builder.executes(self.base.get_command().unwrap());
        }
        builder
    }

    fn get_examples(&self) -> Vec<String> {
        self.type_.get_examples()
    }
}

impl Display for ArgumentCommandNode<'_, String, String> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<argument {}: {}>", self.name, self.type_)
    }
}
