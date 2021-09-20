use std::ops::Deref;

use ecs_lib::World;

fn initialize_world() -> World {
    let mut world = World::new();
    world.add_resource(FpsResource(60));
    world
}

#[test]
fn create_and_get_reource_immutably() {
    let world = initialize_world();
    let fps = world.get_resource::<FpsResource>().unwrap();
    assert_eq!(fps.0, 60);
}

#[test]
fn get_resources_mutably() {
    let mut world = initialize_world();
    {
        let fps: &mut FpsResource = world.get_resource_mut::<FpsResource>().unwrap();
        fps.0 += 1;
    }

    let fps = world.get_resource::<FpsResource>().unwrap();
    assert_eq!(fps.0, 61)
}

struct FpsResource(pub u32);

impl Deref for FpsResource {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
