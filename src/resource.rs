use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

#[derive(Default)]
pub struct Resource {
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl Resource {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, data: impl Any) {
        let type_id = data.type_id();
        self.data.insert(type_id, Box::new(data));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[allow(clippy::clippy::float_cmp)]
    fn add_resource() {
        let mut resources = Resource::new();
        let world_width = WorldWidth(100.0);
        let world_with_type_id = world_width.type_id();

        resources.add(world_width);

        let stored_resource = resources.data.get(&world_with_type_id).unwrap();
        let extracted_world_width = stored_resource.downcast_ref::<WorldWidth>().unwrap();

        assert_eq!(extracted_world_width.0, 100.0);
    }

    struct WorldWidth(pub f32);
}
