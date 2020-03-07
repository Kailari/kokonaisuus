use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;

pub use self::read::Read;
pub use self::write::Write;

mod write;
mod read;

pub struct ComponentStorage {
    capacity: usize,
    components: HashMap<TypeId, Box<dyn Any>>,
}

impl ComponentStorage {
    pub fn new(capacity: usize) -> ComponentStorage {
        ComponentStorage {
            capacity,
            components: HashMap::new(),
        }
    }

    pub fn register_component_type<C>(&mut self)
        where C: Any
    {
        let mut storage_vector = Vec::<Option<C>>::with_capacity(self.capacity);
        storage_vector.resize_with(self.capacity, || None);
        self.components.insert(TypeId::of::<C>(), Box::new(RefCell::new(storage_vector)));
    }

    pub fn add_to_entity<C>(&self, entity_id: usize, component: C)
        where C: Any
    {
        let mut storage = self.fetch_component_storage::<C>().borrow_mut();
        if let Some(Some(_)) = storage.get(entity_id) {
            panic!("Tried adding component {} to entity_id={}, but it already has one!",
                   std::any::type_name::<C>(),
                   entity_id);
        }

        storage[entity_id] = Some(component);
    }

    pub fn fetch_mut<C>(&self) -> Write<C>
        where C: Any
    {
        Write::from(self.fetch_component_storage::<C>().borrow_mut())
    }

    pub fn fetch_ref<C>(&self) -> Read<C>
        where C: Any
    {
        Read::from(self.fetch_component_storage::<C>().borrow())
    }

    fn fetch_component_storage<C>(&self) -> &RefCell<Vec<Option<C>>>
        where C: Any
    {
        let component_type_id = TypeId::of::<C>();
        let storage = self.components.get(&component_type_id)
                          .unwrap_or_else(|| panic!("Use of an unregistered component type: {}",
                                                    std::any::type_name::<C>()));

        storage.downcast_ref::<RefCell<Vec<Option<C>>>>().unwrap()
    }
}
