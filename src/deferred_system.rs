use crate::ecs_hooks::EcsContext;
use bevy::ecs::system::{IntoSystem, Resource, System};

#[derive(Resource, Default)]
pub struct DeferredSystemRunQueue {
    pub run_queue: Box<Vec<Box<dyn System<In = (), Out = ()>>>>,
}

#[derive(Clone, Copy)]
pub struct DeferredSystemScheduler {
    run_queue: *mut Vec<Box<dyn System<In = (), Out = ()>>>,
}

impl DeferredSystemScheduler {
    pub fn schedule<S, M>(&self, system: S)
    where
        S: IntoSystem<(), (), M> + 'static,
        M: 'static,
    {
        unsafe { &mut *self.run_queue }.push(Box::new(S::into_system(system)));
    }
}

unsafe impl Send for DeferredSystemScheduler {}
unsafe impl Sync for DeferredSystemScheduler {}

pub fn use_system_scheduler() -> DeferredSystemScheduler {
    DeferredSystemScheduler {
        run_queue: Box::as_mut(
            &mut EcsContext::get_world()
                .resource_mut::<DeferredSystemRunQueue>()
                .run_queue,
        ),
    }
}
