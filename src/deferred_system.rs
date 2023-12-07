use bevy::ecs::{
    system::{IntoSystem, Resource, SystemId},
    world::World,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct DeferredSystem(pub(crate) Arc<DeferredSystemInner>);

pub(crate) struct DeferredSystemInner {
    pub id: SystemId,
    world: *mut World,
}

impl DeferredSystem {
    pub(crate) fn new<S>(system: S, world: &mut World) -> Self
    where
        S: IntoSystem<(), (), ()> + 'static,
    {
        Self(Arc::new(DeferredSystemInner {
            id: world.register_system(system),
            world,
        }))
    }

    pub fn schedule(&self) {
        unsafe { &mut *self.0.world }
            .resource_mut::<DeferredSystemRunQueue>()
            .0
            .push(self.clone());
    }
}

impl Drop for DeferredSystemInner {
    fn drop(&mut self) {
        unsafe { &mut *self.world }.remove_system(self.id).unwrap();
    }
}

unsafe impl Send for DeferredSystemInner {}
unsafe impl Sync for DeferredSystemInner {}

#[derive(Resource, Default)]
pub struct DeferredSystemRunQueue(pub Vec<DeferredSystem>);
