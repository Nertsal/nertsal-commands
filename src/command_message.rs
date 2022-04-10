use super::AuthorityLevel;

#[derive(Debug, Clone)]
pub struct CommandMessage<S> {
    pub sender: S,
    pub message_text: String,
    pub authority_level: AuthorityLevel,
}
