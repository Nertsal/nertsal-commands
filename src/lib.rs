mod command_message;
mod command_node;

pub use command_message::*;
pub use command_node::*;

pub struct Commands<T, S> {
    pub commands: Vec<CommandNode<T, S>>,
}

impl<T, S> Commands<T, S> {
    pub fn new(commands: Vec<CommandNode<T, S>>) -> Self {
        Self { commands }
    }

    pub fn perform_commands(&self, actor: &mut T, message: &CommandMessage<S>) -> Vec<Response> {
        self.find_commands(message)
            .into_iter()
            .map(|(command, args)| command(actor, &message.sender, args))
            .collect()
    }

    pub fn find_commands(
        &self,
        message: &CommandMessage<S>,
    ) -> Vec<(Command<T, S>, Vec<Argument>)> {
        self.commands
            .iter()
            .filter_map(|com| com.check_node(&message.message_text, Vec::new()))
            .filter_map(|(command, arguments)| match command {
                CommandNode::Final {
                    authority_level,
                    command,
                } => {
                    if check_authority_level(authority_level, &message) {
                        Some((command.clone(), arguments))
                    } else {
                        None
                    }
                }
                _ => unreachable!(),
            })
            .collect()
    }
}

fn check_authority_level<S>(authority_level: &AuthorityLevel, message: &CommandMessage<S>) -> bool {
    message.authority_level >= *authority_level
}
