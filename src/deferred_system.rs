use bevy::ecs::{
    system::{IntoSystem, Resource, SystemId},
    world::World,
};

#[derive(Clone, Copy)]
pub struct DeferredSystem {
    id: SystemId,
    world: *mut World,
}

impl DeferredSystem {
    pub(crate) fn new<S>(system: S, world: &mut World) -> Self
    where
        S: IntoSystem<(), (), ()> + 'static,
    {
        Self {
            id: world.register_system(system),
            world,
        }
    }

    pub fn schedule(&self) {
        unsafe { &mut *self.world }
            .resource_mut::<DeferredSystemRunQueue>()
            .0
            .push(self.id);
    }
}

unsafe impl Send for DeferredSystem {}
unsafe impl Sync for DeferredSystem {}

pub struct OnDropUnregisterDeferredSystem(pub DeferredSystem);

impl Drop for OnDropUnregisterDeferredSystem {
    fn drop(&mut self) {
        unsafe { &mut *self.0.world }
            .remove_system(self.0.id)
            .unwrap();
    }
}

#[derive(Resource, Default)]
pub struct DeferredSystemRunQueue(pub Vec<SystemId>);
