use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{EventId, EventPayload, EventTime, ExternalEvent, ExternalEventKind};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalEventRecord {
    pub event_id: EventId,
    pub event_time: EventTime,
    pub kind: ExternalEventKind,
    pub payload: EventPayload,
    pub payload_hash: String,
}

impl ExternalEventRecord {
    pub fn from_event(event: &ExternalEvent) -> Self {
        let payload_hash = hash_payload(&event.payload);
        Self {
            event_id: event.event_id().clone(),
            event_time: event.at(),
            kind: event.kind(),
            payload: event.payload().clone(),
            payload_hash,
        }
    }

    pub fn rehydrate(&self) -> ExternalEvent {
        ExternalEvent::with_payload(
            self.event_id.clone(),
            self.kind,
            self.event_time,
            self.payload.clone(),
        )
    }

    pub fn validate_hash(&self) -> bool {
        self.payload_hash == hash_payload(&self.payload)
    }
}

pub fn hash_payload(payload: &EventPayload) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&payload.data);
    let digest = hasher.finalize();
    hex::encode(digest)
}
