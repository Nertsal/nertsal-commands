use super::*;

impl<T: ?Sized, S, R> CommandNode<T, S, R> {
    pub fn literal(
        literals: impl IntoIterator<Item = impl Into<String>>,
        children: Vec<CommandNode<T, S, R>>,
    ) -> Self {
        Self::Literal {
            literals: literals.into_iter().map(|literal| literal.into()).collect(),
            child_nodes: children,
        }
    }

    pub fn argument(argument_type: ArgumentType, children: Vec<CommandNode<T, S, R>>) -> Self {
        Self::Argument {
            argument_type,
            child_nodes: children,
        }
    }

    pub fn argument_choice(
        choices: impl IntoIterator<Item = impl Into<String>>,
        children: Vec<CommandNode<T, S, R>>,
    ) -> Self {
        Self::ArgumentChoice {
            choices: choices.into_iter().map(|choice| choice.into()).collect(),
            child_nodes: children,
        }
    }

    pub fn final_node(
        expects_empty_message: bool,
        authority_level: AuthorityLevel,
        command: Command<T, S, R>,
    ) -> Self {
        Self::Final {
            expects_empty_message,
            authority_level,
            command,
        }
    }
}
