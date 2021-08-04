mod command_message;
mod command_node;

use command_message::*;
use command_node::*;

pub struct Commands<T> {
    pub commands: Vec<CommandNode<T>>,
}

impl<T> Commands<T> {
    pub fn perform_commands(&self, actor: &mut T, message: &CommandMessage) -> Vec<Response> {
        self.find_commands(message)
            .into_iter()
            .map(|(command, args)| command(actor, message.sender_name.clone(), args))
            .collect()
    }

    pub fn find_commands(&self, message: &CommandMessage) -> Vec<(Command<T>, Vec<Argument>)> {
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

fn check_authority_level(authority_level: &AuthorityLevel, message: &CommandMessage) -> bool {
    message.authority_level >= *authority_level
}
