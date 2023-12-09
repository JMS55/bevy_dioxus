use crate::{
    deferred_system::{new_deferred_system, DeferredSystem},
    tick::EcsContext,
};
use bevy::ecs::{
    query::{QueryState, ReadOnlyWorldQueryData, WorldQueryFilter},
    system::{IntoSystem, Query, Resource},
    world::{unsafe_world_cell::UnsafeWorldCell, World},
};
use dioxus_core::ScopeState;

pub trait DioxusUiHooks {
    fn use_world<'a>(&'a self) -> &'a World;

    fn use_resource<'a, T: Resource>(&'a self) -> &'a T;

    fn use_query<'a, Q>(&'a self) -> DioxusUiQuery<'a, Q, ()>
    where
        Q: ReadOnlyWorldQueryData;

    fn use_query_filtered<'a, Q, F>(&'a self) -> DioxusUiQuery<'a, Q, F>
    where
        Q: ReadOnlyWorldQueryData,
        F: WorldQueryFilter;

    fn use_system<S>(&self, system: S) -> DeferredSystem
    where
        S: IntoSystem<(), (), ()> + 'static;
}

// TODO: Hooks need to schedule future updates
impl DioxusUiHooks for ScopeState {
    fn use_world<'a>(&'a self) -> &'a World {
        EcsContext::get_world(self)
    }

    fn use_resource<'a, T: Resource>(&'a self) -> &'a T {
        EcsContext::get_world(self).resource()
    }

    fn use_query<'a, Q>(&'a self) -> DioxusUiQuery<'a, Q, ()>
    where
        Q: ReadOnlyWorldQueryData,
    {
        Self::use_query_filtered(self)
    }

    fn use_query_filtered<'a, Q, F>(&'a self) -> DioxusUiQuery<'a, Q, F>
    where
        Q: ReadOnlyWorldQueryData,
        F: WorldQueryFilter,
    {
        let world = EcsContext::get_world(self);
        DioxusUiQuery {
            query_state: QueryState::new(world),
            world_cell: world.as_unsafe_world_cell(),
        }
    }

    fn use_system<S>(&self, system: S) -> DeferredSystem
    where
        S: IntoSystem<(), (), ()> + 'static,
    {
        self.use_hook(|| new_deferred_system(system, EcsContext::get_world(self)))
            .0
    }
}

pub struct DioxusUiQuery<'a, Q: ReadOnlyWorldQueryData, F: WorldQueryFilter> {
    query_state: QueryState<Q, F>,
    world_cell: UnsafeWorldCell<'a>,
}

impl<'a, Q, F> DioxusUiQuery<'a, Q, F>
where
    Q: ReadOnlyWorldQueryData,
    F: WorldQueryFilter,
{
    pub fn query(&self) -> Query<Q, F> {
        unsafe {
            Query::new(
                self.world_cell,
                &self.query_state,
                self.world_cell.last_change_tick(),
                self.world_cell.change_tick(),
                true,
            )
        }
    }
}
