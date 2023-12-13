use bevy::{
    ecs::{
        system::{IntoSystem, Resource, SystemId},
        world::World,
    },
    utils::HashMap,
};
use std::sync::Arc;

#[derive(Resource, Default)]
pub struct DeferredSystemRegistry {
    // TODO: Replace ref_counts with Box<Vec<SystemId>> and insert SystemId into it on unmount
    pub ref_counts: HashMap<SystemId, Arc<()>>,
    pub run_queue: Box<Vec<SystemId>>,
}

#[derive(Clone, Copy)]
struct DeferredSystem {
    id: SystemId,
    run_queue: *mut Vec<SystemId>,
}

impl DeferredSystem {
    fn schedule(&self) {
        unsafe { &mut *self.run_queue }.push(self.id);
    }
}

unsafe impl Send for DeferredSystem {}
unsafe impl Sync for DeferredSystem {}

pub fn new_deferred_system<S, M>(
    system: S,
    world: &mut World,
) -> (impl Fn() + Send + Sync + Copy, Arc<()>)
where
    S: IntoSystem<(), (), M> + 'static,
    M: 'static,
{
    let id = world.register_system(system);
    let ref_count = Arc::new(());

    let mut system_registry = world.resource_mut::<DeferredSystemRegistry>();
    system_registry
        .ref_counts
        .insert(id, Arc::clone(&ref_count));

    let deferred_system = DeferredSystem {
        id,
        run_queue: Box::as_mut(&mut system_registry.run_queue),
    };

    (move || deferred_system.schedule(), ref_count)
}
