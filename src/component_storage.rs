use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;

pub struct ComponentStorage {
    components: HashMap<TypeId, Box<dyn Any>>,
}

impl ComponentStorage {
    pub fn new() -> ComponentStorage {
        ComponentStorage { components: HashMap::new() }
    }

    pub fn register_component_type<C>(&mut self, components: Vec<C>)
        where C: Any
    {
        self.components.insert(TypeId::of::<C>(), Box::new(RefCell::new(components)));
    }

    pub fn fetch_mut<C>(&self) -> RefMut<Vec<C>>
        where C: Any
    {
        self.fetch_component_storage::<C>()
            .borrow_mut()
    }

    pub fn fetch_ref<C>(&self) -> Ref<Vec<C>>
        where C: Any
    {
        self.fetch_component_storage::<C>()
            .borrow()
    }

    fn fetch_component_storage<C>(&self) -> &RefCell<Vec<C>>
        where C: Any
    {
        let component_type_id = TypeId::of::<C>();
        let storage = self.components.get(&component_type_id)
                          .unwrap_or_else(|| panic!("Use of an unregistered component type: {}",
                                                    std::any::type_name::<C>()));

        storage.downcast_ref::<RefCell<Vec<C>>>().unwrap()
    }
}
