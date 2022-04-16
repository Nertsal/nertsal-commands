use super::*;

pub struct CommandBuilder<T, S> {
    nodes: Vec<CommandNode<T, S>>,
}

impl<T, S> CommandBuilder<T, S> {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn literal(mut self, literals: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.nodes.push(CommandNode::literal(literals, vec![]));
        self
    }

    pub fn word(mut self) -> Self {
        self.nodes
            .push(CommandNode::argument(ArgumentType::Word, vec![]));
        self
    }

    pub fn line(mut self) -> Self {
        self.nodes
            .push(CommandNode::argument(ArgumentType::Line, vec![]));
        self
    }

    pub fn choice(mut self, choices: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.nodes
            .push(CommandNode::argument_choice(choices, vec![]));
        self
    }

    pub fn finalize(
        self,
        expects_empty_message: bool,
        authority_level: AuthorityLevel,
        command: Command<T, S>,
    ) -> CommandNode<T, S> {
        let final_node = CommandNode::final_node(expects_empty_message, authority_level, command);
        (std::iter::once(final_node))
            .chain(self.nodes.into_iter().rev())
            .reduce(|child, mut parent| {
                let children = match &mut parent {
                    CommandNode::Literal { child_nodes, .. } => child_nodes,
                    CommandNode::Argument { child_nodes, .. } => child_nodes,
                    CommandNode::ArgumentChoice { child_nodes, .. } => child_nodes,
                    CommandNode::Final { .. } => unreachable!(),
                };
                children.push(child);
                parent
            })
            .unwrap()
    }
}
