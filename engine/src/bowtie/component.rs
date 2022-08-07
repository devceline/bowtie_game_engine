use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

use crate::{general::value::Value, StandardEntity};

pub type ComponentStore = HashMap<String, Value>;
pub type ComponentFunction<'a> =
  dyn Fn(&mut StandardEntity<'a>, Arc<Mutex<ComponentStore>>) -> () + 'a;

#[derive(Clone)]
pub struct StandardComponent<'a> {
  component_function: Arc<ComponentFunction<'a>>,
  name: String,
  store: Arc<Mutex<ComponentStore>>,
}

impl<'a> StandardComponent<'a> {
  pub fn new(
    f: Arc<ComponentFunction<'a>>,
    name: &str,
    store_seed: ComponentStore,
  ) -> StandardComponent<'a> {
    StandardComponent {
      component_function: f,
      name: String::from(name),
      store: Arc::new(Mutex::new(store_seed)),
    }
  }

  pub fn get_store(&self) -> &Arc<Mutex<ComponentStore>> {
    &self.store
  }

  pub fn get_name(&self) -> String {
    self.name.to_owned()
  }

  pub fn act(&self, entity: &mut StandardEntity<'a>) -> () {
    let func = &self.component_function;
    func(entity, self.store.clone());
  }
}
