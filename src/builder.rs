use super::*;

pub struct CommandBuilder<T: ?Sized, S> {
    nodes: Vec<CommandNode<T, S>>,
}

impl<T: ?Sized, S> CommandBuilder<T, S> {
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
        fold_nodes(final_node, self.nodes.into_iter().rev())
    }

    /// Assumes that there at least one node has been inserted before
    pub fn split(self, children: impl IntoIterator<Item = CommandNode<T, S>>) -> CommandNode<T, S> {
        assert!(
            self.nodes.len() > 0,
            "Expected at least one node in the builder"
        );

        let mut nodes = self.nodes.into_iter().rev();
        let mut final_node = nodes.next().unwrap();
        let child_nodes = final_node
            .children_mut()
            .expect("A final node cannot be in the middle");
        child_nodes.extend(children);
        fold_nodes(final_node, nodes)
    }
}

fn fold_nodes<T: ?Sized, S>(
    final_node: CommandNode<T, S>,
    nodes: impl IntoIterator<Item = CommandNode<T, S>>,
) -> CommandNode<T, S> {
    (std::iter::once(final_node))
        .chain(nodes.into_iter())
        .reduce(|child, mut parent| {
            let children = parent
                .children_mut()
                .expect("A final node cannot be in the middle");
            children.push(child);
            parent
        })
        .unwrap()
}
