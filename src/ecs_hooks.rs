use crate::UiContext;
use bevy::{
    ecs::{
        component::ComponentId,
        event::{Event, EventIterator, Events, ManualEventReader},
        query::{QueryFilter, ReadOnlyQueryData},
        system::{Query, Resource, SystemState},
        world::World,
    },
    utils::{HashMap, HashSet},
};
use dioxus::{
    core::{ScopeId, ScopeState},
    hooks::use_on_destroy,
};
use std::any::TypeId;

#[derive(Default)]
pub(crate) struct EcsSubscriptions {
    pub resources: Box<HashMap<ComponentId, HashSet<ScopeId>>>,
    #[allow(clippy::type_complexity)]
    pub events: Box<HashMap<TypeId, (Box<dyn Fn(&World) -> bool>, HashSet<ScopeId>)>>,
    pub world_and_queries: Box<HashSet<ScopeId>>,
}

#[derive(Clone)]
pub(crate) struct EcsContext {
    pub world: *mut World,
}

impl EcsContext {
    #[allow(clippy::mut_from_ref)]
    pub fn get_world(cx: &ScopeState) -> &mut World {
        unsafe {
            &mut *cx
                .consume_context::<EcsContext>()
                .expect("Must be used from a dioxus component within a DioxusUiRoot bevy component")
                .world
        }
    }
}

pub fn use_world(cx: &ScopeState) -> &World {
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

pub fn use_resource<T: Resource>(cx: &ScopeState) -> &T {
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

pub fn use_query<Q>(cx: &ScopeState) -> UseQuery<'_, Q, ()>
where
    Q: ReadOnlyQueryData,
{
    use_query_filtered(cx)
}

pub fn use_query_filtered<Q, F>(cx: &ScopeState) -> UseQuery<'_, Q, F>
where
    Q: ReadOnlyQueryData,
    F: QueryFilter,
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

    UseQuery {
        system_state: SystemState::new(world),
        world_ref: world,
    }
}

pub fn use_event_reader<E: Event>(cx: &ScopeState) -> EventIterator<'_, E> {
    // TODO: Register the subscription

    let event_reader = cx.use_hook(ManualEventReader::default);
    let events = EcsContext::get_world(cx).resource::<Events<E>>();
    event_reader.read(events)
}

pub struct UseQuery<'a, Q: ReadOnlyQueryData + 'static, F: QueryFilter + 'static> {
    system_state: SystemState<Query<'static, 'static, Q, F>>,
    world_ref: &'a World,
}

impl<'a, Q, F> UseQuery<'a, Q, F>
where
    Q: ReadOnlyQueryData,
    F: QueryFilter,
{
    pub fn query(&mut self) -> Query<Q, F> {
        self.system_state.get(self.world_ref)
    }
}
