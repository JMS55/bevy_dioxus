use bevy::ecs::{
    system::{IntoSystem, Resource, SystemId},
    world::World,
};

#[derive(Clone, Copy)]
pub struct DeferredSystem {
    id: SystemId,
    run_queue: *mut Vec<SystemId>,
}

impl DeferredSystem {
    pub(crate) fn new<S>(system: S, world: &mut World) -> Self
    where
        S: IntoSystem<(), (), ()> + 'static,
    {
        Self {
            id: world.register_system(system), // TODO: We never unregister the system
            run_queue: Box::as_mut(&mut world.resource_mut::<DeferredSystemRunQueue>().0),
        }
    }

    pub fn schedule(&self) {
        unsafe { &mut *self.run_queue }.push(self.id);
    }
}

unsafe impl Send for DeferredSystem {}
unsafe impl Sync for DeferredSystem {}

#[derive(Resource, Clone, Default)]
pub struct DeferredSystemRunQueue(pub Box<Vec<SystemId>>);
