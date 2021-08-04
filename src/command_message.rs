use super::AuthorityLevel;

pub struct CommandMessage {
    pub sender_name: String,
    pub message_text: String,
    pub authority_level: AuthorityLevel,
}
