use crate::{
    deferred_system::{new_deferred_system, DeferredSystem},
    tick::EcsContext,
};
use bevy::ecs::{
    query::{QueryState, ReadOnlyWorldQuery},
    system::{IntoSystem, Query, Resource},
    world::{unsafe_world_cell::UnsafeWorldCell, World},
};
use dioxus::core::ScopeState;

// TODO: Hooks need to schedule future updates

pub fn use_world<'a>(cx: &'a ScopeState) -> &'a World {
    EcsContext::get_world(cx)
}

pub fn use_resource<'a, T: Resource>(cx: &'a ScopeState) -> &'a T {
    EcsContext::get_world(cx).resource()
}

pub fn use_query<'a, Q>(cx: &'a ScopeState) -> DioxusUiQuery<'a, Q, ()>
where
    Q: ReadOnlyWorldQuery,
{
    use_query_filtered(cx)
}

pub fn use_query_filtered<'a, Q, F>(cx: &'a ScopeState) -> DioxusUiQuery<'a, Q, F>
where
    Q: ReadOnlyWorldQuery,
    F: ReadOnlyWorldQuery,
{
    let world = EcsContext::get_world(cx);
    DioxusUiQuery {
        query_state: QueryState::new(world),
        world_cell: world.as_unsafe_world_cell(),
    }
}

pub fn use_system<S>(cx: &ScopeState, system: S) -> DeferredSystem
where
    S: IntoSystem<(), (), ()> + 'static,
{
    cx.use_hook(|| new_deferred_system(system, EcsContext::get_world(cx)))
        .0
}

pub struct DioxusUiQuery<'a, Q: ReadOnlyWorldQuery, F: ReadOnlyWorldQuery> {
    query_state: QueryState<Q, F>,
    world_cell: UnsafeWorldCell<'a>,
}

impl<'a, Q, F> DioxusUiQuery<'a, Q, F>
where
    Q: ReadOnlyWorldQuery,
    F: ReadOnlyWorldQuery,
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
