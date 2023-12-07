use bevy::{
    ecs::{
        system::{IntoSystem, Resource, SystemId},
        world::World,
    },
    utils::HashMap,
};
use std::sync::Arc;

#[derive(Clone, Copy)]
pub struct DeferredSystem {
    id: SystemId,
    run_queue: *mut Vec<SystemId>,
}

impl DeferredSystem {
    pub(crate) fn new<S>(system: S, world: &mut World) -> (Self, Arc<()>)
    where
        S: IntoSystem<(), (), ()> + 'static,
    {
        let id = world.register_system(system);
        let ref_count = Arc::new(());

        let mut system_registry = world.resource_mut::<DeferredSystemRegistry>();
        system_registry
            .ref_counts
            .insert(id, Arc::clone(&ref_count));

        let deferred_system = Self {
            id,
            run_queue: Box::as_mut(&mut system_registry.run_queue),
        };
        (deferred_system, ref_count)
    }

    pub fn schedule(&self) {
        unsafe { &mut *self.run_queue }.push(self.id);
    }
}

unsafe impl Send for DeferredSystem {}
unsafe impl Sync for DeferredSystem {}

#[derive(Resource, Default)]
pub struct DeferredSystemRegistry {
    pub ref_counts: HashMap<SystemId, Arc<()>>,
    pub run_queue: Box<Vec<SystemId>>,
}
