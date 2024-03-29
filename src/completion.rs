use super::*;

impl<T: ?Sized, S, R, Term: linefeed::Terminal> linefeed::Completer<Term> for Commands<T, S, R> {
    fn complete(
        &self,
        _word: &str,
        prompter: &linefeed::Prompter<Term>,
        _start: usize,
        _end: usize,
    ) -> Option<Vec<linefeed::Completion>> {
        let message = prompter.buffer();
        Some(
            self.commands
                .iter()
                .flat_map(|node| node.complete(message))
                .collect(),
        )
    }
}

impl<T: ?Sized, S, R> CommandNode<T, S, R> {
    pub fn complete(&self, message: &str) -> Vec<linefeed::Completion> {
        let mut completions = Vec::new();
        match self {
            CommandNode::Literal {
                literals,
                child_nodes,
            } => {
                for literal in literals {
                    if literal.starts_with(message) && literal != message {
                        completions.push(linefeed::Completion::simple(literal.clone()));
                    }
                }
                if let Some(literal) = literals
                    .iter()
                    .find(|&literal| message.starts_with(literal))
                {
                    let message = message[literal.len()..].trim();
                    for child_node in child_nodes {
                        completions.append(&mut child_node.complete(message));
                    }
                }
            }
            CommandNode::Argument {
                argument_type,
                child_nodes,
            } => match argument_type {
                ArgumentType::Word => {
                    if let Some(argument) = message.split_whitespace().next() {
                        let message = message[argument.len()..].trim();
                        for child_node in child_nodes {
                            completions.append(&mut child_node.complete(message));
                        }
                    }
                }
                ArgumentType::Line => (),
            },
            CommandNode::ArgumentChoice {
                choices,
                child_nodes,
            } => {
                for choice in choices {
                    if choice.starts_with(message) && choice != message {
                        completions.push(linefeed::Completion::simple(choice.clone()));
                    }
                }
                if let Some(choice) = choices.iter().find(|&choice| message.starts_with(choice)) {
                    let message = message[choice.len()..].trim();
                    for child_node in child_nodes {
                        completions.append(&mut child_node.complete(message));
                    }
                }
            }
            CommandNode::Final { .. } => (),
        }
        completions
    }
}
