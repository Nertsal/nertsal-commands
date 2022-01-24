use super::*;

impl<T, S> CommandNode<T, S> {
    pub fn literal<'a>(
        literals: impl IntoIterator<Item = &'a str>,
        children: Vec<CommandNode<T, S>>,
    ) -> Self {
        Self::Literal {
            literals: literals
                .into_iter()
                .map(|literal| literal.to_owned())
                .collect(),
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
        choices: impl IntoIterator<Item = &'a str>,
        children: Vec<CommandNode<T, S>>,
    ) -> Self {
        Self::ArgumentChoice {
            choices: choices
                .into_iter()
                .map(|choice| choice.to_owned())
                .collect(),
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