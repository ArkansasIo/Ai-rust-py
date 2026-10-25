use crate::domain::conversation::{Conversation, ConversationContext};

pub struct ConversationService;
impl ConversationService {
    pub fn context(conversation: &Conversation, token_budget: u32) -> ConversationContext {
        ConversationContext { conversation_id: conversation.id, summary: None, token_budget, retrieved_memory_ids: Vec::new() }
    }
}
