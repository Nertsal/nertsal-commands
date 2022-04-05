mod command_message;
mod command_node;
mod completion;
mod constructor;

pub use command_message::*;
pub use command_node::*;
pub use constructor::*;

#[derive(Clone)]
pub struct Commands<T: ?Sized, S> {
    pub commands: Vec<CommandNode<T, S>>,
}

impl<T: ?Sized, S> Commands<T, S> {
    pub fn new(commands: Vec<CommandNode<T, S>>) -> Self {
        Self { commands }
    }

    pub fn perform_commands<'a>(
        &'a self,
        actor: &'a mut T,
        message: &'a CommandMessage<S>,
    ) -> impl Iterator<Item = Response> + 'a {
        self.find_commands(message)
            .map(move |(command, args)| command(actor, &message.sender, args))
    }

    pub fn find_commands<'a>(
        &'a self,
        message: &'a CommandMessage<S>,
    ) -> impl Iterator<Item = (Command<T, S>, Vec<Argument>)> + 'a {
        let message_text = &message.message_text;
        let message_authority_level = message.authority_level;

        self.commands
            .iter()
            .filter_map(move |com| com.check_node(message_text, Vec::new()))
            .filter_map(move |(command, arguments)| match command {
                CommandNode::Final {
                    authority_level,
                    command,
                    ..
                } => {
                    if *authority_level <= message_authority_level {
                        Some((command.clone(), arguments))
                    } else {
                        None
                    }
                }
                _ => unreachable!(),
            })
    }
}
