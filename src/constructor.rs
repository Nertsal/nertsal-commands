use super::*;

impl<T, S> CommandNode<T, S> {
    pub fn literal<'a>(
        literals: impl IntoIterator<Item = impl Into<String>>,
        children: Vec<CommandNode<T, S>>,
    ) -> Self {
        Self::Literal {
            literals: literals.into_iter().map(|literal| literal.into()).collect(),
            child_nodes: children,
        }
    }

    pub fn argument(argument_type: ArgumentType, children: Vec<CommandNode<T, S>>) -> Self {
        Self::Argument {
            argument_type,
            child_nodes: children,
        }
    }

    pub fn argument_choice<'a>(
        choices: impl IntoIterator<Item = impl Into<String>>,
        children: Vec<CommandNode<T, S>>,
    ) -> Self {
        Self::ArgumentChoice {
            choices: choices.into_iter().map(|choice| choice.into()).collect(),
            child_nodes: children,
        }
    }

    pub fn final_node(authority_level: AuthorityLevel, command: Command<T, S>) -> Self {
        Self::Final {
            authority_level,
            command,
        }
    }
}
