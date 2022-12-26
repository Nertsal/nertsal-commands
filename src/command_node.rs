use std::sync::Arc;

#[derive(Clone)]
pub enum CommandNode<T: ?Sized, S, R> {
    Literal {
        literals: Vec<String>,
        child_nodes: Vec<CommandNode<T, S, R>>,
    },
    Argument {
        argument_type: ArgumentType,
        child_nodes: Vec<CommandNode<T, S, R>>,
    },
    ArgumentChoice {
        choices: Vec<String>,
        child_nodes: Vec<CommandNode<T, S, R>>,
    },
    Final {
        /// If `true`, then this node will activate only when the message is fully consumed,
        /// when it reached this node. If `false`, then this node will always activate if reached.
        expects_empty_message: bool,
        authority_level: AuthorityLevel,
        command: Command<T, S, R>,
    },
}

pub type AuthorityLevel = usize;
pub type Argument = String;
pub type Command<T, S, R> = Arc<dyn Fn(&mut T, &S, Vec<Argument>) -> R + Send + Sync>;

#[derive(Debug, Clone, Copy)]
pub enum ArgumentType {
    Word,
    Line,
}

macro_rules! child_nodes {
    ( $child_nodes: expr, $message: expr, $arguments: expr ) => {
        for child_node in $child_nodes {
            if let Some((final_node, arguments)) =
                child_node.check_node($message, $arguments.clone())
            {
                return Some((final_node, arguments));
            }
        }
    };
}

type CheckedNode<'a, T, S, R> = (&'a CommandNode<T, S, R>, Vec<Argument>);

impl<T: ?Sized, S, R> CommandNode<T, S, R> {
    pub fn check_node(
        &self,
        message: &str,
        mut arguments: Vec<Argument>,
    ) -> Option<CheckedNode<T, S, R>> {
        match self {
            CommandNode::Literal {
                literals,
                child_nodes,
            } => literals
                .iter()
                .find(|&literal| message.starts_with(literal))
                .and_then(|literal| {
                    let message = message[literal.len()..].trim();
                    child_nodes!(child_nodes, message, arguments);
                    None
                }),

            CommandNode::Argument {
                argument_type,
                child_nodes,
            } => match argument_type {
                ArgumentType::Word => message.split_whitespace().next(),
                ArgumentType::Line => {
                    if message.trim().is_empty() {
                        None
                    } else {
                        Some(message)
                    }
                }
            }
            .and_then(|argument| {
                let message = message[argument.len()..].trim();
                arguments.push(argument.to_owned());
                child_nodes!(child_nodes, message, arguments);
                None
            }),

            CommandNode::ArgumentChoice {
                choices,
                child_nodes,
            } => choices
                .iter()
                .find(|choice| message.starts_with(*choice))
                .and_then(|choice| {
                    let message = message[choice.len()..].trim();
                    arguments.push(choice.to_owned());
                    child_nodes!(child_nodes, message, arguments);
                    None
                }),

            &CommandNode::Final {
                expects_empty_message,
                ..
            } => {
                if expects_empty_message && !message.trim().is_empty() {
                    None
                } else {
                    Some((self, arguments))
                }
            }
        }
    }

    pub fn children_mut(&mut self) -> Option<&mut Vec<CommandNode<T, S, R>>> {
        match self {
            Self::Literal { child_nodes, .. } => Some(child_nodes),
            Self::Argument { child_nodes, .. } => Some(child_nodes),
            Self::ArgumentChoice { child_nodes, .. } => Some(child_nodes),
            Self::Final { .. } => None,
        }
    }
}
