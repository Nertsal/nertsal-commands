use std::sync::Arc;

#[derive(Clone)]
pub enum CommandNode<T: ?Sized, S> {
    Literal {
        literals: Vec<String>,
        child_nodes: Vec<CommandNode<T, S>>,
    },
    Argument {
        argument_type: ArgumentType,
        child_nodes: Vec<CommandNode<T, S>>,
    },
    ArgumentChoice {
        choices: Vec<String>,
        child_nodes: Vec<CommandNode<T, S>>,
    },
    Final {
        /// If `true`, then this node will activate only when the message is fully consumed,
        /// when it reached this node. If `false`, then this node will always activate if reached.
        expects_empty_message: bool,
        authority_level: AuthorityLevel,
        command: Command<T, S>,
    },
}

pub type AuthorityLevel = usize;
pub type Argument = String;
pub type Response = Option<String>;
pub type Command<T, S> = Arc<dyn Fn(&mut T, &S, Vec<Argument>) -> Response + Send + Sync>;

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

impl<T: ?Sized, S> CommandNode<T, S> {
    pub fn check_node(
        &self,
        message: &str,
        mut arguments: Vec<Argument>,
    ) -> Option<(&CommandNode<T, S>, Vec<Argument>)> {
        match self {
            CommandNode::Literal {
                literals,
                child_nodes,
            } => literals
                .iter()
                .find(|&literal| message.starts_with(literal))
                .map(|literal| {
                    let message = message[literal.len()..].trim();
                    child_nodes!(child_nodes, message, arguments);
                    None
                })
                .flatten(),

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
            .map(|argument| {
                let message = message[argument.len()..].trim();
                arguments.push(argument.to_owned());
                child_nodes!(child_nodes, message, arguments);
                None
            })
            .flatten(),

            CommandNode::ArgumentChoice {
                choices,
                child_nodes,
            } => choices
                .iter()
                .find(|choice| message.starts_with(*choice))
                .map(|choice| {
                    let message = message[choice.len()..].trim();
                    arguments.push(choice.to_owned());
                    child_nodes!(child_nodes, message, arguments);
                    None
                })
                .flatten(),

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

    pub fn children_mut(&mut self) -> Option<&mut Vec<CommandNode<T, S>>> {
        match self {
            Self::Literal { child_nodes, .. } => Some(child_nodes),
            Self::Argument { child_nodes, .. } => Some(child_nodes),
            Self::ArgumentChoice { child_nodes, .. } => Some(child_nodes),
            Self::Final { .. } => None,
        }
    }
}
