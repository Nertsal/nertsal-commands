use super::*;

/// Helps to easier create trees of [CommandNode]. Makes construction of
/// deep command trees more concise. Conversely, constructing
/// wide trees takes about as much words as with using
/// constructors of [CommandNode]. Most helper functions in this struct
/// allow constructing only 1-wide trees (effectively lists),
/// but when you need to split a node, you can use the `split` function.
pub struct CommandBuilder<T: ?Sized, S> {
    nodes: Vec<CommandNode<T, S>>,
}

impl<T: ?Sized, S> CommandBuilder<T, S> {
    /// Initializes a builder
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Adds a literal node that accepts specific literals
    pub fn literal(mut self, literals: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.nodes.push(CommandNode::literal(literals, vec![]));
        self
    }

    /// Adds a literal node that expects an word argument
    pub fn word(mut self) -> Self {
        self.nodes
            .push(CommandNode::argument(ArgumentType::Word, vec![]));
        self
    }

    /// Adds a literal node that takes a line as an argument
    pub fn line(mut self) -> Self {
        self.nodes
            .push(CommandNode::argument(ArgumentType::Line, vec![]));
        self
    }

    /// Adds a literal node that accepts only certain literals
    /// and forwards the chosen one as an argument
    pub fn choice(mut self, choices: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.nodes
            .push(CommandNode::argument_choice(choices, vec![]));
        self
    }

    /// Finalizes the branch by adding a final node
    pub fn finalize(
        self,
        expects_empty_message: bool,
        authority_level: AuthorityLevel,
        command: Command<T, S>,
    ) -> CommandNode<T, S> {
        let final_node = CommandNode::final_node(expects_empty_message, authority_level, command);
        fold_nodes(final_node, self.nodes.into_iter().rev())
    }

    /// Split the branch into several. Useful when several commands
    /// start from the same pattern.
    /// Assumes that at least one node has been inserted before
    pub fn split(self, children: impl IntoIterator<Item = CommandNode<T, S>>) -> CommandNode<T, S> {
        assert!(
            !self.nodes.is_empty(),
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

impl<T: ?Sized, S> Default for CommandBuilder<T, S> {
    fn default() -> Self {
        Self::new()
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

#[macro_export]
macro_rules! command {
    // Final
    ($empty:expr, $authority:expr, $function:expr) => {{
        CommandNode::final_node($empty, $authority, $function)
    }};
    // Literal
    ($($literals:literal),+; $($tail:tt)*) => {{
        let children = vec![command!($($tail)*)];
        CommandNode::literal([$($literals),+], children)
    }};
    // Choice
    ($($choices:literal)|+; $($tail:tt)*) => {{
        let children = vec![command!($($tail)*)];
        CommandNode::argument_choice([$($choices),+], children)
    }};
    // Argument word
    (word; $($tail:tt)*) => {{
        let children = vec![command!($($tail)*)];
        CommandNode::argument(ArgumentType::Word, children)
    }};
    // Argument line
    (line; $($tail:tt)*) => {{
        let children = vec![command!($($tail)*)];
        CommandNode::argument(ArgumentType::Line, children)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_command_macro() {
        // `CommandNode::final_node(...)`
        command!(true, 0, Arc::new(|_: &mut (), _: &(), _| None));

        // `CommandBuilder::new().literal(["!help"]).finalize(...)`
        command!(
            "!help";
            true, 0, Arc::new(|_: &mut (), _: &(), _| None)
        );

        // `CommandBuilder::new().literal(["!backup"]).choice(["create", "load"]).finalize(...)`
        command!(
            "!backup";
            "create" | "load";
            true, 0, Arc::new(|_: &mut (), _: &(), _| None)
        );

        // `CommandBuilder::new().literal(["!hello"]).word().finalize(...)`
        command!(
            "!hello";
            word;
            true, 0, Arc::new(|_: &mut (), _: &(), _| None)
        );

        // `CommandBuilder::new().literal(["!echo"]).line().finalize(...)`
        command!(
            "!echo";
            line;
            true, 0, Arc::new(|_: &mut (), _: &(), _| None)
        );
    }
}
