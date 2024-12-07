use std::cell::RefCell;
use std::rc::Rc;

use std::collections::HashMap;

use crate::{general::value::Value, StandardEntity};

pub type ComponentStore = HashMap<String, Value>;
pub type ComponentFunction =
  dyn Fn(&mut StandardEntity, Rc<RefCell<ComponentStore>>) -> ();

#[derive(Clone)]
pub struct StandardComponent {
  component_function: Rc<ComponentFunction>,
  name: String,
  store: Rc<RefCell<ComponentStore>>,
}

impl StandardComponent {
  pub fn new(
    f: Rc<ComponentFunction>,
    name: &str,
    store_seed: ComponentStore,
  ) -> StandardComponent {
    StandardComponent {
      component_function: Rc::clone(&f),
      name: String::from(name),
      store: Rc::new(RefCell::new(store_seed)),
    }
  }

  pub fn get_store(&self) -> Rc<RefCell<ComponentStore>> {
    Rc::clone(&self.store)
  }

  pub fn get_name(&self) -> String {
    self.name.to_owned()
  }

  pub fn act(&self, entity: &mut StandardEntity) -> () {
    let func = &self.component_function;
    func(entity, self.store.clone());
  }
}
