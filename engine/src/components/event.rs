use std::collections::HashMap;

use crate::bowtie::entity::{Component, Entity, Message};

pub struct EventComponent {
  events: HashMap<*mut dyn Entity, Vec<Message>>,
}

impl EventComponent {
  pub fn new() -> EventComponent {
    EventComponent {
      events: HashMap::new(),
    }
  }

  pub fn push_message(&mut self, entity: *mut dyn Entity, message: Message) {
    let event_messages = self.events.entry(entity).or_insert(Vec::new());

    (*event_messages).push(message);
  }
}

impl Component for EventComponent {
  fn get_name(&self) -> &str {
    "event"
  }

  unsafe fn act(
    &mut self,
    _entities: &Vec<*mut dyn Entity>,
    entity: *mut dyn Entity,
  ) -> Option<Message> {
    let event_messages = self.events.entry(entity).or_insert(Vec::new());

    event_messages.pop()
  }
}
