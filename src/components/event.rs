use std::collections::HashMap;

use crate::bowtie::entity::{Component, Entity, Message};

pub struct EventComponent<'s> {
  events: HashMap<*mut dyn Entity<'s>, Vec<Message>>,
}

impl<'s> EventComponent<'s> {
  pub fn new() -> EventComponent<'s> {
    EventComponent {
      events: HashMap::new(),
    }
  }

  pub fn push_message(&mut self, entity: *mut dyn Entity<'s>, message: Message) {
    let event_messages = self.events.entry(entity).or_insert(Vec::new());

    (*event_messages).push(message);
  }
}

impl<'s> Component<'s> for EventComponent<'s> {
  fn get_name(&self) -> &str {
    "event"
  }

  unsafe fn act(
    &mut self,
    _entities: &Vec<*mut dyn Entity<'s>>,
    entity: *mut dyn Entity<'s>,
  ) -> Option<Message> {
    let event_messages = self.events.entry(entity).or_insert(Vec::new());

    event_messages.pop()
  }
}
