use super::cognitive_event::CognitiveEvent;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct CognitiveBus {
    sender: broadcast::Sender<CognitiveEvent>,
}

impl CognitiveBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }
    pub fn publish(&self, event: CognitiveEvent) -> Result<usize, broadcast::error::SendError<CognitiveEvent>> { self.sender.send(event) }
    pub fn subscribe(&self) -> broadcast::Receiver<CognitiveEvent> { self.sender.subscribe() }
}
