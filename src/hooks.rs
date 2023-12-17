use crate::{deferred_system::new_deferred_system, UiContext};
use bevy::{
    ecs::{
        component::ComponentId,
        query::{QueryState, ReadOnlyWorldQuery},
        system::{IntoSystem, Query, Resource},
        world::{unsafe_world_cell::UnsafeWorldCell, World},
    },
    utils::{HashMap, HashSet},
};
use dioxus::{
    core::{ScopeId, ScopeState},
    hooks::use_on_destroy,
};

#[derive(Default)]
pub(crate) struct EcsSubscriptions {
    pub resources: Box<HashMap<ComponentId, HashSet<ScopeId>>>,
    pub world_and_queries: Box<HashSet<ScopeId>>,
}

#[derive(Clone)]
pub(crate) struct EcsContext {
    pub world: *mut World,
}

impl EcsContext {
    fn get_world(cx: &ScopeState) -> &mut World {
        unsafe {
            &mut *cx
                .consume_context::<EcsContext>()
                .expect("Must be used from a dioxus component within a DioxusUiRoot bevy component")
                .world
        }
    }
}

pub fn use_world<'a>(cx: &'a ScopeState) -> &'a World {
    let world = EcsContext::get_world(cx);

    let scope_id = cx.scope_id();
    let subscription_manager = *cx.use_hook(|| {
        let subscription_manager = &mut world
            .non_send_resource_mut::<UiContext>()
            .subscriptions
            .world_and_queries;
        subscription_manager.insert(scope_id);
        Box::as_mut(subscription_manager) as *mut HashSet<ScopeId>
    });
    use_on_destroy(cx, move || {
        unsafe { &mut *subscription_manager }.remove(&scope_id);
    });

    world
}

pub fn use_resource<'a, T: Resource>(cx: &'a ScopeState) -> &'a T {
    let world = EcsContext::get_world(cx);

    let resource_id = world.components().resource_id::<T>().unwrap();
    let scope_id = cx.scope_id();
    let subscription_manager = *cx.use_hook(|| {
        let subscription_manager = &mut world
            .non_send_resource_mut::<UiContext>()
            .subscriptions
            .resources;
        subscription_manager
            .entry(resource_id)
            .or_default()
            .insert(scope_id);
        Box::as_mut(subscription_manager) as *mut HashMap<ComponentId, HashSet<ScopeId>>
    });
    use_on_destroy(cx, move || {
        let subscription_manager = &mut unsafe { &mut *subscription_manager };
        let resource_subscriptions = subscription_manager.get_mut(&resource_id).unwrap();
        resource_subscriptions.remove(&scope_id);
        if resource_subscriptions.is_empty() {
            subscription_manager.remove(&resource_id);
        }
    });

    world.resource()
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

    let scope_id = cx.scope_id();
    let subscription_manager = *cx.use_hook(|| {
        let subscription_manager = &mut world
            .non_send_resource_mut::<UiContext>()
            .subscriptions
            .world_and_queries;
        subscription_manager.insert(scope_id);
        Box::as_mut(subscription_manager) as *mut HashSet<ScopeId>
    });
    use_on_destroy(cx, move || {
        unsafe { &mut *subscription_manager }.remove(&scope_id);
    });

    DioxusUiQuery {
        query_state: QueryState::new(world),
        world_cell: world.as_unsafe_world_cell(),
    }
}

pub fn use_system<S, M>(cx: &ScopeState, system: S) -> impl Fn() + Send + Sync + Copy
where
    S: IntoSystem<(), (), M> + 'static,
    M: 'static,
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
