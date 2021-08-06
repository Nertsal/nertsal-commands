use super::AuthorityLevel;

pub struct CommandMessage<S> {
    pub sender: S,
    pub message_text: String,
    pub authority_level: AuthorityLevel,
}
