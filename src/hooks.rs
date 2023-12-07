use crate::{
    deferred_system::{DeferredSystem, OnDropUnregisterDeferredSystem},
    tick::EcsContext,
};
use bevy::ecs::{
    system::{IntoSystem, Resource},
    world::World,
};
use dioxus_core::ScopeState;

pub trait DioxusUiHooks {
    fn use_world<'a>(&'a self) -> &'a World;
    fn use_resource<'a, T: Resource>(&'a self) -> &'a T;
    fn use_system<S>(&self, system: S) -> DeferredSystem
    where
        S: IntoSystem<(), (), ()> + 'static;
}

impl DioxusUiHooks for ScopeState {
    fn use_world<'a>(&'a self) -> &'a World {
        EcsContext::get_world(self)
    }

    fn use_resource<'a, T: Resource>(&'a self) -> &'a T {
        EcsContext::get_world(self).resource()
    }

    fn use_system<S>(&self, system: S) -> DeferredSystem
    where
        S: IntoSystem<(), (), ()> + 'static,
    {
        self.use_hook(|| {
            OnDropUnregisterDeferredSystem(DeferredSystem::new(system, EcsContext::get_world(self)))
        })
        .0
    }
}

// TODO
// pub fn use_query<'a, Q, F>(cx: &'a ScopeState) -> QueryIter<'a, '_, Q, F>
// where
//     Q: ReadOnlyWorldQuery,
//     F: ReadOnlyWorldQuery,
// {
//     let world = EcsContext::get_world(cx);
//     world.query_filtered::<Q, F>().iter(&world)
// }